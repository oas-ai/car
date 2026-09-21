//! Genesis G80 2017 decoded signal을 Canonical VehicleState로 변환한다.

use oas_can::decode::{DecodedCanMessage, SignalValue};

use crate::adapter::ManufacturerAdapter;
use crate::vehicle_state::VehicleState;

const KPH_TO_MPS: f64 = 1.0 / 3.6;
const MPH_TO_MPS: f64 = 0.447_04;
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

/// Genesis G80 2017의 read-only 상태 adapter다.
#[derive(Debug, Default)]
pub struct GenesisG80Adapter {
    state: VehicleState,
}

impl GenesisG80Adapter {
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
}

impl ManufacturerAdapter for GenesisG80Adapter {
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
                if let Some(driver_override) = Self::number(message, "DriverOverride") {
                    self.state.brake.pressed = Some(driver_override == 2.0);
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
