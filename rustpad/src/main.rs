use gilrs::{Button, Event, EventType, Gilrs};

const LEFT_AXIS_CODE: u32 = 5;
const RIGHT_AXIS_CODE: u32 = 2;
enum ThrustmasterEvent{

}
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
            match event {
                EventType::ButtonPressed(_, code) => {
                    println!("ButtonPressed, code: {:?}", code);
                    match code.into_u32(){
                        0 => {}
                        _ => {}
                    }
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
                EventType::Connected => {
                    println!("new device connected")
                }
                EventType::Disconnected => {}
                EventType::Dropped => {
                    println!("event dropped!")
                }
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
fn decode(event : Event) -> Event{
    match event.event{
        EventType::ButtonPressed(_, _) => {},
        EventType::ButtonRepeated(_, _) => {},
        EventType::ButtonReleased(_, _) => {},
        EventType::ButtonChanged(_, _, _) => {},
        EventType::AxisChanged(_, _, _) => {},
        EventType::Connected => {},
        EventType::Disconnected => {},
        EventType::Dropped => {},
    }
}