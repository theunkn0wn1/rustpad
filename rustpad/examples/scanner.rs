use gilrs::{Button, Event, EventType, Gilrs};
use rustpad::thrustmaster::codes;
use rustpad::thrustmaster::decode_warthog_throttle;
use std::fs::File;

const LEFT_AXIS_CODE: u32 = 5;
const RIGHT_AXIS_CODE: u32 = 2;

fn main() {
    println!("Hello, world!");
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!(
            "{}[{:?}] is {:?}",
            gamepad.name(),
            gamepad.uuid(),
            gamepad.power_info()
        );
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                EventType::ButtonPressed(_, code) | EventType::ButtonReleased(_, code) => {
                    if let Some(decoded_event) = decode_warthog_throttle(event) {
                        println!("successful decode {:?}", decoded_event);
                    } else {
                        println!("event {:?}", event)
                    };
                    // let data = serde_json::to_string(&code).expect("failed decode");
                    // println!("==\n{:?}\n===", data);

                    // let decoded: gilrs::ev::Code = serde_json::from_str(&data).unwrap();
                    // println!("decoded: {:?}", decoded)
                }
                EventType::ButtonRepeated(_, code) => {
                    println!("ButtonRepeated, code: {:?}", code);
                }
                EventType::ButtonChanged(_, value, code) => {
                    // println!("ButtonChanged, code: {:?}, value: {:?}", code, value, )
                }
                EventType::AxisChanged(_, value, code) => {
                    println!("Axis movement, new_value: {:?}, code: {:?}", value, code)
                }
                EventType::Connected => {}
                EventType::Disconnected => {}
                EventType::Dropped => {}
            }
            active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            let state = gamepad.state();

            if let Some(button) = state.button_data(codes::get_code(codes::Inputs::EacArm)) {
                if button.is_pressed() {
                    println!("at {:?} EacARm is pressed!", button.timestamp())
                }
            }
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }
    }
}
