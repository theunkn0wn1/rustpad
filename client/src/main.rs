use tokio;
use rustpad::thrustmaster::warthog_stick::codes;
use gilrs::Gilrs;
use std::collections::HashMap;
use tokio::time::Duration;
use rover_tonic::borealis::{ArmState, GripState};


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut gilrs = Gilrs::new().unwrap();
    let mut ids: std::vec::Vec<gilrs::GamepadId> = std::vec::Vec::new();

    // Iterate over all connected gamepads
    for (id, gamepad) in gilrs.gamepads() {
        println!(
            "{}[{:?}] is {:?} with id {:?}",
            gamepad.name(),
            gamepad.uuid(),
            gamepad.power_info(),
            gamepad.id(),
        );
        ids.push(id);
    }


    loop {
        // nom all event data
        while let Some(_) = gilrs.next_event() {}

        if let Some(pad) = gilrs.connected_gamepad(*ids.get(0).unwrap()) {
            let grip_state = process_gripper(& pad);
            println!("rotation := {}, pitch := {}, grip := {}", grip_state.rotation_axis, grip_state.pitch_axis, grip_state.grip_axis);
        } else { println!("device not found!") }
    }

    fn process_gripper(pad: &gilrs::Gamepad) -> GripState {
        let paddle_code = codes::get_code(codes::Inputs::PaddlePressed);
        let trigger_code = codes::get_code(codes::Inputs::TriggerSoftPressed);
        let x_code = codes::get_code(codes::Inputs::ScXAction);
        let y_code = codes::get_code(codes::Inputs::ScYAction);

        let rotation_value = if let Some(data) = pad.state().axis_data(x_code) {
            data.value()
        } else { 0.0 };
        let pitch_value = if let Some(data) = pad.state().axis_data(y_code) {
            data.value()
        } else { 0.0 };
        let grip_trigger = if pad.state().is_pressed(trigger_code)
        {
            1.0
        } else { 0.0 };

        rover_tonic::borealis::GripState {
            grip_axis: grip_trigger,
            rotation_axis: rotation_value,
            pitch_axis: pitch_value,
        }
    }
}
