mod immortality_factory;

use immortality_factory::prelude::*;
use std::{array, fs};

fn all_items() -> World {
    let mut row = World::new();
    let mut vaults = vec![];
    for (i, item) in Item::ITEMS.iter().copied().enumerate() {
        vaults.push(row.place_structure_with_data(
            StructureData::StorageVault {
                input: item,
                storage: [item; 16],
                output: item,
            },
            (i * 5) as i32,
            0,
        ));
    }
    let mut grid = World::new();
    let mut prev = None;
    for i in 0..16 {
        let cur = grid.paste(&row, 0, i * 2);
        if let Some(prev) = &prev {
            for v in vaults.iter() {
                grid.connect(v.inside(prev).output(0), v.inside(&cur).input(0));
            }
        }
        prev = Some(cur);
    }
    grid
}

fn main() {
    let (mana_refinery, mr_w, mr_h, mr_out) = {
        let mut bp = World::new();
        let pump = bp.place_structure(AirPump, 0, 0);
        let refinery = bp.place_structure(Refinery, 2, 0);
        bp.connect(pump.output(0), refinery.input(0));
        (bp, 8, 2, refinery.output(0))
    };
    let (stack, st_w, st_h, st_in, st_out) = {
        let mut col = World::new();
        let merge = col.place_structure(BigMerger, 8, 2);
        for i in 0..4 {
            let mr = col.paste(&mana_refinery, 0, (i as i32) * mr_h);
            col.connect(mr.get(mr_out), merge.input(i + 1))
        }

        let mut bp = World::new();
        let a = bp.paste(&col, 0, 0);
        let b = bp.paste(&col, 9, 0);
        bp.connect(merge.inside(&a).output(0), merge.inside(&b).input(0));
        let c = bp.paste(&col, 18, 0);
        bp.connect(merge.inside(&b).output(0), merge.inside(&c).input(0));
        (
            bp,
            27,
            8,
            merge.inside(&a).input(0),
            merge.inside(&c).output(0),
        )
    };

    let mut world = World::new();
    world.place_structure(Laboratory, 0, -2);
    world.paste(&all_items(), -100, -100);

    let a = world.paste(&stack, 0, 0);
    let b = world.paste(&stack, 0, st_h);
    world.connect(st_out.inside(&a), st_in.inside(&b));

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
