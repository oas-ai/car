use oas_can::decode::{DecodeContext, FrameDecoder};
use oas_can::frame::{CanFrame, CanId};
use oas_can::genesis_g80::GenesisG80Decoder;
use oas_car::adapter::ManufacturerAdapter;
use oas_car::genesis_g80::GenesisG80Adapter;

fn decode(frame: CanFrame) -> oas_can::decode::DecodedCanMessage {
    GenesisG80Decoder
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
    let cluster = decode(CanFrame::new(
        CanId::standard(1265).unwrap(),
        vec![0, 160, 0, 0],
        false,
    )
    .unwrap());
    let steering = decode(CanFrame::new(
        CanId::standard(688).unwrap(),
        vec![132, 3, 0, 0, 0],
        false,
    )
    .unwrap());
    let braking = decode(CanFrame::new(
        CanId::standard(916).unwrap(),
        vec![0, 0, 0, 0, 0, 64, 0, 0],
        false,
    )
    .unwrap());

    let mut adapter = GenesisG80Adapter::default();
    adapter.apply(&cluster).unwrap();
    adapter.apply(&steering).unwrap();
    adapter.apply(&braking).unwrap();

    let state = adapter.vehicle_state();
    assert!((state.vehicle_speed_mps.unwrap() - 80.0 / 3.6).abs() < 0.000_001);
    assert!(
        (state.steering.angle_rad.unwrap() - std::f32::consts::FRAC_PI_2).abs() < 0.000_001
    );
    assert_eq!(state.brake.pressed, Some(true));
    assert!(state.is_fresh_at(1_050, 50));
}
