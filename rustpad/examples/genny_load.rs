use std::fs::File;
use std::io::Read;
use askama::Template;
use rustpad::generator::{DeviceDescriptor, ModuleTemplate};

fn main() {
    let mut file = File::open("./warthog_throttle.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let data: DeviceDescriptor = toml::from_str(&contents).unwrap();

    let template = ModuleTemplate {
        button_events: &data.buttons,
        two_way_events: &data.two_way,
        three_way_events: &data.three_way
    };
    println!("{}", template.render().unwrap());
}