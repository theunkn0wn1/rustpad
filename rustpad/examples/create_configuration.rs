use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use toml::to_string_pretty;

use rustpad::generator::{
    AxisEvent, ButtonEvent, DeviceDescriptor, Event, ThreeWaySwitchEvent, TwoWaySwitchEvent,
};
use toml::ser::Error;

fn main() {
    let mut data = DeviceDescriptor {
        name: "Thrustmaster Warthog Throttle".to_string(),
        id: "44F0404".to_string(),
        axes: vec![
            AxisEvent {
                action: Event {
                    code: 0,
                    name: "ScXAction".to_string(),
                },
            },
            AxisEvent {
                action: Event {
                    code: 1,
                    name: "ScYAction".to_string(),
                },
            },
            AxisEvent {
                action: Event {
                    code: 2,
                    name: "ThrottleR".to_string(),
                },
            },
            AxisEvent {
                action: Event {
                    code: 5,
                    name: "ThrottleL".to_string(),
                },
            },
            AxisEvent {
                action: Event {
                    code: 6,
                    name: "Slew".to_string(),
                },
            },
        ],
        triggers: Default::default(),
        buttons: vec![
            ButtonEvent {
                code: 713,
                pressed_name: "AutopilotTogglePressed".to_string(),
                released_name: "AutopilotToggleReleased".to_string(),
            },
            ButtonEvent {
                code: 708,
                pressed_name: "LandingGearSilencePressed".to_string(),
                released_name: "LandingGearSilenceReleased".to_string(),
            },
            ButtonEvent {
                code: 302,
                pressed_name: "LtbPressed".to_string(),
                released_name: "LtbReleased".to_string(),
            },
        ],
        two_way: vec![
            TwoWaySwitchEvent {
                code: 711,
                high: "EacArm".to_string(),
                neutral: "EacOff".to_string(),
            },
            TwoWaySwitchEvent {
                code: 712,
                high: "RadarAltimeterNormal".to_string(),
                neutral: "RadarAltimeterDisarm".to_string(),
            },
            TwoWaySwitchEvent {
                code: 707,
                high: "ApuStart".to_string(),
                neutral: "ApuOff".to_string(),
            },
            TwoWaySwitchEvent {
                code: 303,
                high: "EngineLeftNormal".to_string(),
                neutral: "EngineLeftOverride".to_string(),
            },
            TwoWaySwitchEvent {
                code: 704,
                high: "EngineRightNormal".to_string(),
                neutral: "EngineRightOverride".to_string(),
            },
        ],
        three_way: vec![
            ThreeWaySwitchEvent {
                neutral: "AltitudeHeading".to_string(),
                high: Event {
                    code: 714,
                    name: "Path".to_string(),
                },
                low: Event {
                    code: 715,
                    name: "Alt".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "Maneuver".to_string(),
                high: Event {
                    code: 709,
                    name: "Up".to_string(),
                },
                low: Event {
                    code: 710,
                    name: "Down".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "LeftNormal".to_string(),
                high: Event {
                    code: 718,
                    name: "LeftIgnition".to_string(),
                },
                low: Event {
                    code: 705,
                    name: "LeftMotor".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "RightNormal".to_string(),
                high: Event {
                    code: 719,
                    name: "RightIgnition".to_string(),
                },
                low: Event {
                    code: 706,
                    name: "RightMotor".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "PsNeutral".to_string(),
                high: Event {
                    code: 300,
                    name: "PsForward".to_string(),
                },
                low: Event {
                    code: 301,
                    name: "PsBackward".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "SpdNeutral".to_string(),
                high: Event {
                    code: 294,
                    name: "SpdForward".to_string(),
                },
                low: Event {
                    code: 295,
                    name: "SpdBackward".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "BsNeutral".to_string(),
                high: Event {
                    code: 296,
                    name: "BsForward".to_string(),
                },
                low: Event {
                    code: 297,
                    name: "BsBackward".to_string(),
                },
            },
            ThreeWaySwitchEvent {
                neutral: "ChNeutral".to_string(),
                high: Event {
                    code: 298,
                    name: "ChForward".to_string(),
                },
                low: Event {
                    code: 299,
                    name: "ChBackward".to_string(),
                },
            },
        ],
    };
    let mut ofile = OpenOptions::new()
        .create(true)
        .write(true)
        .open("output.toml")
        .unwrap();
    match to_string_pretty(&data) {
        Ok(data) => {
            ofile.write(&data.into_bytes()).unwrap();
        }
        Err(_) => {}
    }
    println!("{}", to_string_pretty(&data).unwrap());
}
