use gilrs::{Button, Event, EventType, Gilrs};

const LEFT_AXIS_CODE: u32 = 5;
const RIGHT_AXIS_CODE: u32 = 2;

#[derive(Debug)]
enum WarthogThrottleEvent {
    AutopilotTogglePressed,
    AutopilotToggleReleased,
    LTBPressed,
    LTBReleased,
    LandingGearSilencePressed,
    LandingGearSilenceReleased,
    ScXAction,
    ScYAction,
    SlewAction,
    ThrottleLAction,
    ThrottleRAction,
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
                EventType::ButtonPressed(_, code) | EventType::ButtonReleased(_, code)=> {
                    let decoded = decode(event);
                    match decoded {
                        Some(decoded_event) => println!("successfully decode {:?}", decoded_event),
                        None => println!("failed decode on {:?}", event),
                    }
                }

                EventType::ButtonRepeated(_, code) => {
                    println!("ButtonRepeated, code: {:?}", code)
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

const BUTTON_KIND: u32 = 1 << 16;

fn decode(event: EventType) -> Option<WarthogThrottleEvent> {
    match event {
        EventType::ButtonPressed(_, code) => {
            match code.into_u32() {
                // note these values are calculated from underlying  (kind<<16 | hardware_code)
                66249u32 => { Some(WarthogThrottleEvent::AutopilotTogglePressed) }
                66244u32 => { Some(WarthogThrottleEvent::LandingGearSilencePressed) }
                65838u32 => { Some(WarthogThrottleEvent::LTBPressed) }
                _ => { None }
            }
        }
        EventType::ButtonRepeated(_, code) => {
            unimplemented!()
        }
        EventType::ButtonReleased(_, code) => {
            match code.into_u32() {
                // note these values are calculated from underlying  (kind<<16 | hardware_code)
                66249u32 => { Some(WarthogThrottleEvent::AutopilotToggleReleased) }
                66244u32 => { Some(WarthogThrottleEvent::LandingGearSilenceReleased) }
                65838u32 => { Some(WarthogThrottleEvent::LTBReleased) }
                _ => { None }
            }
        }
        EventType::ButtonChanged(_, value, code) => {
            unimplemented!()
        }
        EventType::AxisChanged(_, value, code) => {
            unimplemented!()
        }
        EventType::Connected => {
            None
        }
        EventType::Disconnected => {
            None
        }
        EventType::Dropped => {
            None
        }
    }
}

