use std::fs::File;
use std::io::Read;

use rustpad::generator::Raw;

fn main() {
    let mut file = File::open("./warthog_throttle.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data: Raw = toml::from_str(&contents).unwrap();
    println!("{:?}", data.axes);
}