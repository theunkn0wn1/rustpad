use gilrs::{Button, Event, EventType, Gilrs};

use rustpad::thrustmaster;


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
            if let Some(decode_ev) = thrustmaster::decode_warthog_throttle(event) {
                println!("decode :: {:?}", decode_ev)
            } else{
                match event{
                    EventType::Connected | EventType::Disconnected | EventType::Dropped=> {
                        println!("{:?} New event from {}: {:?}", time, id, event);
                    },
                    _ => {}
                }
            }
            active_gamepad = Some(id);
        }

        // // You can also use cached gamepad state
        // if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
        //     if gamepad.is_pressed(Button::South) {
        //         println!("Button South is pressed (XBox - A, PS - X)");
        //     }
        // }
    }
}
