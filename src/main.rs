use std::{
    fs::{self, File},
    io::Write,
};

use structure::Structure::*;
use world::World;

mod items;
mod structure;
mod world;

fn main() {
    let mut world = World::new();
    world.place_structure(Laboratory, 0, 0);
    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
