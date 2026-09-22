use oas_can::decode::{DecodeContext, FrameDecoder};
use oas_can::frame::{CanFrame, CanId};
use oas_can::hyundai_palisade_2020::HyundaiPalisade2020Decoder;
use oas_car::adapter::ManufacturerAdapter;
use oas_car::hyundai_palisade_2020::HyundaiPalisade2020Adapter;
use oas_car::vehicle_state::GearPosition;

fn decode(frame: CanFrame) -> oas_can::decode::DecodedCanMessage {
    HyundaiPalisade2020Decoder
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
fn palisade_frames_flow_to_canonical_vehicle_state() {
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
    let wheels = decode(
        CanFrame::new(
            CanId::standard(902).unwrap(),
            vec![0, 32, 0, 16, 0, 8, 0, 4],
            false,
        )
        .unwrap(),
    );
    let cruise = decode(
        CanFrame::new(
            CanId::standard(905).unwrap(),
            vec![0, 0, 0, 0, 1, 0, 0, 0],
            false,
        )
        .unwrap(),
    );
    let doors = decode(
        CanFrame::new(
            CanId::standard(1313).unwrap(),
            vec![57, 0, 0, 0, 0, 0, 0, 0],
            false,
        )
        .unwrap(),
    );
    let climate = decode(
        CanFrame::new(
            CanId::standard(66).unwrap(),
            vec![12, 0, 16, 0, 0, 0, 0, 0],
            false,
        )
        .unwrap(),
    );

    let mut adapter = HyundaiPalisade2020Adapter::default();
    adapter.apply(&cluster).unwrap();
    adapter.apply(&steering).unwrap();
    adapter.apply(&accelerating_and_braking).unwrap();
    adapter.apply(&lighting).unwrap();
    adapter.apply(&gear).unwrap();
    adapter.apply(&wheels).unwrap();
    adapter.apply(&cruise).unwrap();
    adapter.apply(&doors).unwrap();
    adapter.apply(&climate).unwrap();

    let state = adapter.vehicle_state();
    assert!((state.vehicle_speed_mps.unwrap() - 80.0 / 3.6).abs() < 0.000_01);
    assert!((state.steering.angle_rad.unwrap() - std::f32::consts::FRAC_PI_2).abs() < 0.000_001);
    assert!((state.acceleration_mps2.unwrap() - 1.25).abs() < 0.000_01);
    assert_eq!(state.brake.pressed, Some(true));
    assert_eq!(state.night_mode, Some(true));
    assert_eq!(state.gear.position, GearPosition::Park);
    assert_eq!(state.wheels.len(), 4);
    assert_eq!(state.wheels[0].speed_mps, Some(256.0 / 3.6));
    assert_eq!(state.wheels[3].speed_mps, Some(32.0 / 3.6));
    assert_eq!(state.cruise.enabled, Some(true));
    assert_eq!(state.raw_signals["GW_DDM_PE.C_DRVDoorStatus"], 1.0);
    assert_eq!(state.raw_signals["GW_DDM_PE.C_RLDoorStatus"], 3.0);
    assert_eq!(state.raw_signals["DATC12.CR_Datc_DrTempDispC"], 20.0);
    assert_eq!(state.raw_signals["DATC12.CR_Datc_PsTempDispC"], 22.0);
    assert!(state.is_fresh_at(1_050, 50));
}
