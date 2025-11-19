use std::fs;

use item::Item::{self, *};
use structure::Structure::{self, *};
use world::{StructureId, World};

mod item;
mod structure;
mod world;

fn storage() -> World {
    let mut row = World::new();
    for (i, item) in Item::ITEMS.iter().copied().enumerate() {
        row.place_structure(StorageVault(Box::new([item; 16])), (i * 6) as i32, 0);
    }
    let mut storage = World::new();
    storage.stack(&row, 0, 0, 0, 2, 16);
    storage
}

fn main() {
    let mut mana_refinery = World::new();
    let pump = mana_refinery.place_structure(AirPump, 0, 0);
    let refinery = mana_refinery.place_structure(Refinery(Box::new([Empty; 12])), 2, 0);
    mana_refinery.connect(pump.output(0), refinery.input(0));

    let mut world = World::new();
    world.place_structure(Laboratory, 0, -2);

    let stack = world.stack(&mana_refinery, 0, 0, 0, 2, 4);
    let merge = world.place_structure(BigMerger, 8, 2);
    for (i, pasted_world) in stack.into_iter().enumerate() {
        world.connect(
            pasted_world.get_in_host(refinery).output(0),
            merge.input(i + 1),
        );
    }

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
