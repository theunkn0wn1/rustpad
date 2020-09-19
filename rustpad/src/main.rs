use gilrs::{Button, Event, EventType, Gilrs};


use crate::thrustmaster::{decode_warthog_throttle, WarthogThrottleEvent};

use std::collections::HashMap;
use toml;
use rustpad::thrustmaster;

fn gamepad_worker() {
    println!("Hello, world!");
    let mut gilrs = Gilrs::new().unwrap();

    let mut keymap: HashMap<String, gilrs::Gamepad> = HashMap::new();
    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!(
            "{}[{:?}] is {:?}",
            gamepad.name(),
            gamepad.uuid(),
            gamepad.power_info(),
        );
        keymap.insert(gamepad.name().to_string(), gamepad);
    }

    loop {
        // Examine new events
        while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                EventType::ButtonPressed(_, code)
                | EventType::ButtonReleased(_, code)
                | EventType::AxisChanged(_, _, code) => {
                    let encoded = toml::to_string_pretty(&code).unwrap();
                    println!("{}", encoded);
                    if let Some(decoded_event) = decode_warthog_throttle(event) {
                        println!("successful decode {:?}", decoded_event);
                    } else {
                        println!("event {:?}", event)
                    };
                }

                EventType::ButtonRepeated(_, code) => println!("ButtonRepeated, code: {:?}", code),
                EventType::ButtonChanged(_, _value, _code) => {
                    // println!("ButtonChanged, code: {:?}, value: {:?}", code, value, )
                }

                EventType::Connected => println!("new device connected"),
                EventType::Disconnected => unimplemented!(),
                EventType::Dropped => println!("event dropped!"),
            }
            // Gilrs suggests calling this at the end of processing an event
            gilrs.inc();
        }
    }
}
fn main() {
    println!("hello world!");
    gamepad_worker();
}
