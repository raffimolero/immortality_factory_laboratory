use std::fs;

use item::Item::{self, *};
use structure::{StructureKind::*, StructureWithData};
use world::{blueprint::Entity, StructureId, World};

mod item;
mod structure;
mod world;

fn storage() -> World {
    let mut row = World::new();
    let mut vaults = vec![];
    for (i, item) in Item::ITEMS.iter().copied().enumerate() {
        vaults.push(row.place_structure_with_data(
            StructureWithData::StorageVault(Box::new([item; 16])),
            (i * 6) as i32,
            0,
        ));
    }
    let mut prev = None;
    let mut storage = World::new();
    for i in 0..16 {
        let cur = storage.paste(&row, 0, i * 2);
        if let Some(prev) = &prev {
            for v in vaults.iter() {
                storage.connect(v.inside_of(prev).output(0), v.inside_of(&cur).input(0));
            }
        }
        prev = Some(cur);
    }
    storage
}

fn main() {
    let mut mana_refinery = World::new();
    let pump = mana_refinery.place_structure(AirPump, 0, 0);
    let refinery = mana_refinery.place_structure(Refinery, 2, 0);
    mana_refinery.connect(pump.output(0), refinery.input(0));

    let mut world = World::new();
    world.place_structure(Laboratory, 0, -2);
    world.paste(&storage(), 0, 0);

    // let stack = world.stack(&mana_refinery, 0, 0, 0, 2, 4);
    // let merge = world.place_structure(BigMerger, 8, 2);
    // for (i, pasted_world) in stack.into_iter().enumerate() {
    //     world.connect(
    //         refinery.inside_of(&pasted_world).output(0),
    //         merge.input(i + 1),
    //     );
    // }

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
