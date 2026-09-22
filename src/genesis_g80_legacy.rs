//! Genesis G80 2017 legacy decoded signal을 Canonical VehicleState로 변환한다.

use oas_can::decode::{DecodedCanMessage, SignalValue};

use crate::adapter::ManufacturerAdapter;
use crate::vehicle_state::{GearPosition, VehicleState};

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
}

impl ManufacturerAdapter for GenesisG80LegacyAdapter {
    type Error = ();

    fn apply(&mut self, message: &DecodedCanMessage) -> Result<(), Self::Error> {
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
            _ => return Ok(()),
        }
        self.state.timestamp_ns = message.context.timestamp_ns;
        Ok(())
    }

    fn vehicle_state(&self) -> &VehicleState {
        &self.state
    }
}
