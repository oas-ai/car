use oas_can::decode::{DecodeContext, FrameDecoder, SignalValue};
use oas_can::fixture::{FixtureDecoder, FixtureMessage};
use oas_can::frame::{CanFrame, CanId};
use oas_car::adapter::ManufacturerAdapter;
use oas_car::vehicle_state::VehicleState;

#[derive(Default)]
struct SyntheticAdapter {
    state: VehicleState,
}

impl ManufacturerAdapter for SyntheticAdapter {
    type Error = ();

    fn apply(&mut self, message: &oas_can::decode::DecodedCanMessage) -> Result<(), Self::Error> {
        if message.message_name != "synthetic_status" {
            return Ok(());
        }

        self.state.timestamp_ns = message.context.timestamp_ns;
        if let Some(SignalValue::Number(speed_mps)) = message.signals.get("speed_mps") {
            self.state.vehicle_speed_mps = Some(*speed_mps as f32);
        }
        if let Some(SignalValue::Boolean(brake_pressed)) = message.signals.get("brake_pressed") {
            self.state.brake.pressed = Some(*brake_pressed);
        }

        Ok(())
    }

    fn vehicle_state(&self) -> &VehicleState {
        &self.state
    }
}

#[test]
fn synthetic_frame_flows_to_a_fresh_vehicle_state() {
    let frame_id = CanId::standard(0x123).unwrap();
    let mut decoder = FixtureDecoder::default();
    decoder.insert(
        frame_id,
        FixtureMessage::new("synthetic_status")
            .with_signal("speed_mps", SignalValue::Number(12.5))
            .with_signal("brake_pressed", SignalValue::Boolean(false)),
    );

    let frame = CanFrame::new(frame_id, vec![0; 8], false).unwrap();
    let message = decoder
        .decode(
            &frame,
            DecodeContext {
                timestamp_ns: Some(1_000),
                bus: 0,
            },
        )
        .unwrap()
        .unwrap();

    let mut adapter = SyntheticAdapter::default();
    adapter.apply(&message).unwrap();

    assert_eq!(adapter.vehicle_state().vehicle_speed_mps, Some(12.5));
    assert_eq!(adapter.vehicle_state().brake.pressed, Some(false));
    assert!(adapter.vehicle_state().is_fresh_at(1_050, 50));
}
