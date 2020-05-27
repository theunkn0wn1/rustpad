use std::collections::HashMap;

use askama::Template;
use serde_derive::{Deserialize, Serialize};
use toml::Value;
use toml::value::{Array, Table};

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
