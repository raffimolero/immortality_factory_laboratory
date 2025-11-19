use std::fs;

use item::Item::{self, *};
use structure::Structure::{self, *};
use world::World;

mod item;
mod structure;
mod world;

fn main() {
    let mut mana_refinery = World::new();
    let pump = mana_refinery.place_structure(AirPump, 0, 0);
    let refinery = mana_refinery.place_structure(Refinery(Box::default()), 2, 0);
    mana_refinery.connect(pump, 0, refinery, 0);

    let mut world = World::new();
    world.place_structure(Laboratory, 0, -2);
    let storage = world.place_structure(StorageVault(Box::new([RichAir; 16])), 0, 0);
    // let stack = world.stack(&mana_refinery, 0, 0, 0, 2, 4);
    // let merge = world.place_structure(BigMerger, 8, 0);
    // for (i, pasted_world) in stack.into_iter().enumerate() {
    //     world.connect(pasted_world.get_in_host(refinery), 0, merge, i);
    // }

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
