use oas_can::decode::{DecodeContext, FrameDecoder};
use oas_can::frame::{CanFrame, CanId};
use oas_can::genesis_g80_legacy::GenesisG80LegacyDecoder;
use oas_car::adapter::ManufacturerAdapter;
use oas_car::genesis_g80_legacy::GenesisG80LegacyAdapter;
use oas_car::vehicle_state::GearPosition;

fn decode(frame: CanFrame) -> oas_can::decode::DecodedCanMessage {
    GenesisG80LegacyDecoder
        .decode(
            &frame,
            DecodeContext {
                timestamp_ns: Some(1_000),
                bus: 0,
            },
        )
        .unwrap()
        .unwrap()
}

#[test]
fn genesis_frames_flow_to_canonical_vehicle_state() {
    let cluster =
        decode(CanFrame::new(CanId::standard(1265).unwrap(), vec![0, 160, 0, 0], false).unwrap());
    let steering =
        decode(CanFrame::new(CanId::standard(688).unwrap(), vec![132, 3, 0, 0, 0], false).unwrap());
    let accelerating_and_braking = decode(
        CanFrame::new(
            CanId::standard(916).unwrap(),
            vec![0, 0, 0, 0, 124, 68, 0, 0],
            false,
        )
        .unwrap(),
    );
    let lighting = decode(
        CanFrame::new(
            CanId::standard(1345).unwrap(),
            vec![0, 0, 0, 128, 0, 0, 0, 0],
            false,
        )
        .unwrap(),
    );
    let gear = decode(
        CanFrame::new(
            CanId::standard(871).unwrap(),
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            false,
        )
        .unwrap(),
    );

    let mut adapter = GenesisG80LegacyAdapter::default();
    adapter.apply(&cluster).unwrap();
    adapter.apply(&steering).unwrap();
    adapter.apply(&accelerating_and_braking).unwrap();
    adapter.apply(&lighting).unwrap();
    adapter.apply(&gear).unwrap();

    let state = adapter.vehicle_state();
    assert!((state.vehicle_speed_mps.unwrap() - 80.0 / 3.6).abs() < 0.000_01);
    assert!((state.steering.angle_rad.unwrap() - std::f32::consts::FRAC_PI_2).abs() < 0.000_001);
    assert!((state.acceleration_mps2.unwrap() - 1.25).abs() < 0.000_01);
    assert_eq!(state.brake.pressed, Some(true));
    assert_eq!(state.night_mode, Some(true));
    assert_eq!(state.gear.position, GearPosition::Park);
    assert!(state.is_fresh_at(1_050, 50));
}
