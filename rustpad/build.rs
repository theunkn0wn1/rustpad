use askama::Template;
use std::fs::File;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};
use toml::value::{Array, Table};
use toml::Value;

#[derive(Deserialize, Debug, Serialize)]
pub struct DeviceDescriptor {
    pub name: String,
    pub id: String,
    pub axes: Vec<AxisEvent>,
    pub buttons: Vec<ButtonEvent>,
    pub two_way: Vec<TwoWaySwitchEvent>,
    pub three_way: Vec<ThreeWaySwitchEvent>,
    pub triggers: Option<Vec<AxisEvent>>,
    pub hats: Option<Vec<HatEvent>>,
}

impl DeviceDescriptor {
    pub fn from_toml(input: PathBuf) -> Self {
        let mut contents = String::new();
        let mut file = OpenOptions::new().read(true).open(input).unwrap();
        file.read_to_string(&mut contents).unwrap();
        let data: DeviceDescriptor = toml::from_str(&contents).unwrap();
        data
    }
    pub fn generate_module(self, output: PathBuf) {
        let template = ModuleTemplate {
            button_events: &self.buttons,
            two_way_events: &self.two_way,
            three_way_events: &self.three_way,
            axes: &self.axes,
        };
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(output)
            .unwrap();
        let out: String = template.render().unwrap();
        file.write_all(out.as_ref()).unwrap()
    }
}

#[derive(Template)]
#[template(path = "mod.rs.txt")]
pub struct ModuleTemplate<'a> {
    pub button_events: &'a Vec<ButtonEvent>,
    pub two_way_events: &'a Vec<TwoWaySwitchEvent>,
    pub three_way_events: &'a Vec<ThreeWaySwitchEvent>,
    pub axes: &'a Vec<AxisEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub code: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AxisEvent {
    pub action: Event,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonEvent {
    pub code: u32,
    pub pressed_name: String,
    pub released_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwoWaySwitchEvent {
    pub code: u32,
    pub high: String,
    pub neutral: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeWaySwitchEvent {
    pub neutral: String,
    pub high: Event,
    pub low: Event,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HatEvent {
    pub name: String,
    pub north: u32,
    pub south: u32,
    pub west: u32,
    pub east: u32,
}


fn main() {
    println!("cargo:rerun-if-changed=warthog_throttle.toml");
    let out_dir = std::env::var(("OUT_DIR")).unwrap();

    let input = PathBuf::from("warthog_throttle.toml");
    let data: DeviceDescriptor = DeviceDescriptor::from_toml(input);
    let output = PathBuf::from(format!("{}/thrustmaster.rs", out_dir));
    // let output = PathBuf::from("src/thrustmaster.rs");
    data.generate_module(output);
}
