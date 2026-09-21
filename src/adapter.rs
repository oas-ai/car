//! DBC decode 결과를 Canonical VehicleState로 변환하는 경계다.

use oas_can::decode::DecodedCanMessage;

use crate::vehicle_state::VehicleState;

/// 제조사·플랫폼별 read-only 상태 변환기다.
pub trait ManufacturerAdapter {
    type Error;

    /// 하나의 DBC decoded message를 상태에 반영한다.
    fn apply(&mut self, message: &DecodedCanMessage) -> Result<(), Self::Error>;

    /// 현재 Canonical VehicleState를 반환한다.
    fn vehicle_state(&self) -> &VehicleState;
}
