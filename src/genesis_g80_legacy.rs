//! Genesis G80 2017 legacy decoded signal을 Canonical VehicleState로 변환한다.

use oas_can::decode::{DecodedCanMessage, SignalValue};

use crate::adapter::ManufacturerAdapter;
use crate::vehicle_state::{GearPosition, VehicleState, WheelPosition, WheelState};

const KPH_TO_MPS: f64 = 1.0 / 3.6;
const MPH_TO_MPS: f64 = 0.447_04;
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// Genesis G80 2017의 read-only 상태 adapter다.
#[derive(Debug, Default)]
pub struct GenesisG80LegacyAdapter {
    state: VehicleState,
}

impl GenesisG80LegacyAdapter {
    fn number(message: &DecodedCanMessage, signal: &str) -> Option<f64> {
        match message.signals.get(signal) {
            Some(SignalValue::Number(value)) => Some(*value),
            _ => None,
        }
    }

    fn unit(message: &DecodedCanMessage) -> Option<&str> {
        match message.signals.get("CF_Clu_SPEED_UNIT") {
            Some(SignalValue::Enumeration(value)) => Some(value),
            _ => None,
        }
    }

    fn gear_position(value: f64) -> GearPosition {
        match value as u8 {
            0 => GearPosition::Park,
            7 => GearPosition::Reverse,
            6 => GearPosition::Neutral,
            5 | 8 => GearPosition::Drive,
            _ => GearPosition::Unspecified,
        }
    }

    fn cruise_enabled(value: f64) -> Option<bool> {
        match value as u8 {
            1 => Some(true),
            0 | 2..=4 => Some(false),
            _ => None,
        }
    }

    fn set_wheel_speed(&mut self, position: WheelPosition, speed_kph: f64) {
        let speed_mps = (speed_kph * KPH_TO_MPS) as f32;
        if let Some(wheel) = self
            .state
            .wheels
            .iter_mut()
            .find(|wheel| wheel.position == position)
        {
            wheel.speed_mps = Some(speed_mps);
        } else {
            self.state.wheels.push(WheelState {
                position,
                speed_mps: Some(speed_mps),
            });
        }
    }
}

impl ManufacturerAdapter for GenesisG80LegacyAdapter {
    type Error = ();

    fn apply(&mut self, message: &DecodedCanMessage) -> Result<(), Self::Error> {
        for (signal, value) in &message.signals {
            if let SignalValue::Number(value) = value {
                self.state
                    .raw_signals
                    .insert(format!("{}.{}", message.message_name, signal), *value);
            }
        }
        match message.message_name.as_str() {
            "CLU11" => {
                let Some(speed) = Self::number(message, "CF_Clu_Vanz") else {
                    return Ok(());
                };
                let speed_mps = match Self::unit(message) {
                    Some("kph") => speed * KPH_TO_MPS,
                    Some("mph") => speed * MPH_TO_MPS,
                    _ => return Ok(()),
                };
                self.state.vehicle_speed_mps = Some(speed_mps as f32);
            }
            "SAS11" => {
                if let Some(angle_deg) = Self::number(message, "SAS_Angle") {
                    self.state.steering.angle_rad = Some((angle_deg * DEG_TO_RAD) as f32);
                }
            }
            "TCS13" => {
                if let Some(acceleration) = Self::number(message, "ACCEL_REF_ACC") {
                    self.state.acceleration_mps2 = Some(acceleration as f32);
                }
                if let Some(driver_override) = Self::number(message, "DriverOverride") {
                    self.state.brake.pressed = Some(driver_override == 2.0);
                }
            }
            "CGW1" => {
                self.state.night_mode =
                    Self::number(message, "CF_Gway_HeadLampLow").map(|value| value != 0.0);
            }
            "LVR12" => {
                if let Some(gear) = Self::number(message, "CF_Lvr_Gear") {
                    self.state.gear.position = Self::gear_position(gear);
                }
            }
            "WHL_SPD11" => {
                for (signal, position) in [
                    ("WHL_SPD_FL", WheelPosition::FrontLeft),
                    ("WHL_SPD_FR", WheelPosition::FrontRight),
                    ("WHL_SPD_RL", WheelPosition::RearLeft),
                    ("WHL_SPD_RR", WheelPosition::RearRight),
                ] {
                    if let Some(speed_kph) = Self::number(message, signal) {
                        self.set_wheel_speed(position, speed_kph);
                    }
                }
            }
            "SCC14" => {
                self.state.cruise.enabled =
                    Self::number(message, "ACCMode").and_then(Self::cruise_enabled);
            }
            _ => return Ok(()),
        }
        self.state.timestamp_ns = message.context.timestamp_ns;
        Ok(())
    }

    fn vehicle_state(&self) -> &VehicleState {
        &self.state
    }
}
