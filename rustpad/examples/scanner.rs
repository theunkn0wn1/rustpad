use gilrs::{Button, Event, EventType, Gilrs};
const LEFT_AXIS_CODE: u32 = 5;
const RIGHT_AXIS_CODE: u32 = 2;

fn main() {
    println!("Hello, world!");
    let mut gilrs = Gilrs::new().unwrap();

// Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{}[{:?}] is {:?}", gamepad.name(), gamepad.uuid(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                EventType::ButtonPressed(_, code) => {
                    println!("ButtonPressed, code: {:?}", code)
                }
                EventType::ButtonRepeated(_, code) => {
                    println!("ButtonRepeated, code: {:?}", code)
                }
                EventType::ButtonReleased(_, code) => {
                    println!("ButtonReleased, code: {:?}", code)
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
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }
    }
}
