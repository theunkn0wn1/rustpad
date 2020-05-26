use std::fs::File;
use std::io::Read;

use toml::to_string_pretty;

use rustpad::generator::{AxisEvent, ButtonEvent, Event, Raw, ThreeWaySwitchEvent, TwoWaySwitchEvent};

fn main() {
    let mut data = Raw {
        axes: vec![
            AxisEvent { action: Event { code: 0, name: "ScXAction".to_string() } },
            AxisEvent { action: Event { code: 1, name: "ScYAction".to_string() } },
        ],
        triggers: Default::default(),
        buttons: vec![
            ButtonEvent {
                pressed: Event { code: 713, name: "AutoPilotToggle".to_string() },
                released: Event { code: 708, name: "LandingGearSilence".to_string() },
            }
        ],
        two_way: vec![
            TwoWaySwitchEvent {
                neutral: Event { code: 0, name: "ApuOff".to_string() },
                high: Event { code: 707, name: "ApuOn".to_string() },
            }
        ],
        three_way: vec![
            ThreeWaySwitchEvent {
                neutral: Event { code: 0, name: "AltitudeHeading".to_string() },
                high: Event { code: 714, name: "Path".to_string() },
                low: Event { code: 715, name: "Altitude".to_string() },
            }
        ],
    };

    println!("{}", to_string_pretty(&data).unwrap());
}