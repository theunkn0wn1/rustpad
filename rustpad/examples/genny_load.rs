use std::fs::File;
use std::io::Read;
use askama::Template;
use rustpad::generator::{DeviceDescriptor, ModuleTemplate};
use std::path::{Path, PathBuf};

fn main() {
    let input = PathBuf::from("./warthog_throttle.toml");
    let data: DeviceDescriptor = DeviceDescriptor::from_toml(input);
    let output = PathBuf::from("./src/thrustmaster.rs");
    data.generate_module(output);
}