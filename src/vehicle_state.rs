//! OEM 신호와 분리된 OAS read-only 차량 상태 모델이다.

/// OAS Canonical Vehicle Model의 read-only 상태 snapshot이다.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VehicleState {
    pub timestamp_ns: Option<u64>,
    pub vehicle_speed_mps: Option<f32>,
    pub acceleration_mps2: Option<f32>,
    pub wheels: Vec<WheelState>,
    pub steering: SteeringState,
    pub brake: BrakeState,
    pub accelerator: AcceleratorState,
    pub gear: GearState,
    pub cruise: CruiseState,
    pub doors: Vec<DoorState>,
    pub seatbelts: Vec<SeatbeltState>,
}

impl VehicleState {
    /// 관측 시각이 지정된 최대 age 안에 있는지 확인한다.
    pub fn is_fresh_at(&self, now_ns: u64, maximum_age_ns: u64) -> bool {
        self.timestamp_ns
            .and_then(|timestamp_ns| now_ns.checked_sub(timestamp_ns))
            .is_some_and(|age_ns| age_ns <= maximum_age_ns)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WheelState {
    pub position: WheelPosition,
    pub speed_mps: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SteeringState {
    pub angle_rad: Option<f32>,
    pub torque_nm: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BrakeState {
    pub pressed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AcceleratorState {
    pub position: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GearPosition {
    #[default]
    Unspecified,
    Park,
    Reverse,
    Neutral,
    Drive,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GearState {
    pub position: GearPosition,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CruiseState {
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoorPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoorState {
    pub position: DoorPosition,
    pub open: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeatPosition {
    Driver,
    FrontPassenger,
    RearLeft,
    RearRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeatbeltState {
    pub position: SeatPosition,
    pub latched: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::VehicleState;

    #[test]
    fn default_state_preserves_unknown_signals() {
        let state = VehicleState::default();

        assert_eq!(state.vehicle_speed_mps, None);
        assert!(state.wheels.is_empty());
    }

    #[test]
    fn state_freshness_requires_a_recent_non_future_timestamp() {
        let state = VehicleState {
            timestamp_ns: Some(100),
            ..VehicleState::default()
        };

        assert!(state.is_fresh_at(150, 50));
        assert!(!state.is_fresh_at(151, 50));
        assert!(!state.is_fresh_at(99, 50));
    }
}
