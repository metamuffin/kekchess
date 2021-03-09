#![feature(exclusive_range_pattern)]
#![allow(dead_code, unused_variables, unused_imports)]

mod chess;
mod interactive;
use chess::{Coord, Game};

fn main() {
    let mut shell = interactive::InteractiveMode::new();
    shell.launch();
}
