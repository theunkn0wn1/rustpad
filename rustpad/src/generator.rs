use std::collections::HashMap;
use std::io::Read;

use askama::Template;
use serde_derive::{Deserialize, Serialize};
use toml::Value;
use toml::value::{Array, Table};
use std::path::PathBuf;
use std::fs::OpenOptions;

#[derive(Deserialize, Debug, Serialize)]
pub struct DeviceDescriptor {
    pub name: String,
    pub id: String,
    pub axes: Vec<AxisEvent>,
    pub triggers: Table,
    pub buttons: Vec<ButtonEvent>,
    pub two_way: Vec<TwoWaySwitchEvent>,
    pub three_way: Vec<ThreeWaySwitchEvent>,
}

impl DeviceDescriptor {

    pub fn from_toml(input: PathBuf) -> Self{
        let mut contents = String::new();
        let mut file = OpenOptions::new().read(true).open(input).unwrap();
        file.read_to_string(&mut contents).unwrap();
        let data: DeviceDescriptor = toml::from_str(&contents).unwrap();
        data
    }
}

#[derive(Template)]
#[template(path = "mod.rs.txt")]
pub struct ModuleTemplate<'a> {
    pub button_events: &'a Vec<ButtonEvent>,
    pub two_way_events: &'a Vec<TwoWaySwitchEvent>,
    pub three_way_events: &'a Vec<ThreeWaySwitchEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub code: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AxisEvent {
    pub action: Event
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
