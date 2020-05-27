use std::fs::File;
use std::io::Read;
use askama::Template;
use rustpad::generator::{DeviceDescriptor, ModuleTemplate};
use std::path::{Path, PathBuf};

fn main() {
    let input = PathBuf::from("./warthog_throttle.toml");
    let data: DeviceDescriptor = DeviceDescriptor::from_toml(input);
    let template = ModuleTemplate {
        button_events: &data.buttons,
        two_way_events: &data.two_way,
        three_way_events: &data.three_way
    };
    println!("{}", template.render().unwrap());
}