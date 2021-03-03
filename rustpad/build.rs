use rustpad_generator::generator::DeviceDescriptor;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=warthog_throttle.toml");
    println!("cargo:rerun-if-changed=warthog_stick.toml");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = "src/thrustmaster";

    let input = PathBuf::from("warthog_throttle.toml");
    let data: DeviceDescriptor = DeviceDescriptor::from_toml(input);
    let output = PathBuf::from(format!("{}/warthog_throttle.rs", out_dir));
    data.generate_module(output);

    let input = PathBuf::from("warthog_stick.toml");
    let data: DeviceDescriptor = DeviceDescriptor::from_toml(input);
    let output = PathBuf::from(format!("{}/warthog_stick.rs", out_dir));
    data.generate_module(output);
}
