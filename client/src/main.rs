use tokio;
use rustpad::thrustmaster::warthog_stick::codes;
use gilrs::Gilrs;
use std::collections::HashMap;
use tokio::time::Duration;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut gilrs = Gilrs::new().unwrap();
    let mut ids :std::vec::Vec<gilrs::GamepadId> = std::vec::Vec::new();

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
    let paddle_code = codes::get_code(codes::Inputs::PaddlePressed);
    let trigger_code = codes::get_code(codes::Inputs::TriggerSoftPressed);
    let x_code = codes::get_code(codes::Inputs::ScXAction);
    let y_code = codes::get_code(codes::Inputs::ScYAction);

    loop {
        // nom all event data
        while let Some(_) = gilrs.next_event() {}

        if let Some(pad) = gilrs.connected_gamepad(*ids.get(0).unwrap()){
            let button = codes::get_code(codes::Inputs::PaddlePressed);
            if  pad.state().is_pressed(button) {
               println!("Pinkie pressed!") ;
            };
            println!("Axis X := {:?} Axis Y := {:?}",
                     pad.state().axis_data(x_code),
                     pad.state().axis_data(y_code)
            );

        } else {println!("device not found!")}

    }
}
