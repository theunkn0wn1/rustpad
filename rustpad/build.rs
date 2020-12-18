use rustpad_generator::generator::DeviceDescriptor;
use std::path::PathBuf;


fn main() {
    println!("cargo:rerun-if-changed=warthog_throttle.toml");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let input = PathBuf::from("warthog_throttle.toml");
    let data: DeviceDescriptor = DeviceDescriptor::from_toml(input);
    let output = PathBuf::from(format!("{}/thrustmaster.rs", out_dir));
    // let output = PathBuf::from("src/thrustmaster.rs");
    data.generate_module(output);
}
