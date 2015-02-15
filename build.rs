#![feature(process)]

use std::process::Command;

fn main() {
    Command::new("jsx").args(&["content/src/", "content/build"])
                       .status().unwrap();
}