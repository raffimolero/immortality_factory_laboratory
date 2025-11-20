mod immortality_factory;

use immortality_factory::prelude::*;
use std::{array, fs, mem::discriminant};

use crate::immortality_factory::structure::Size;

fn all_items() -> World {
    let mut row = World::new();
    let mut vaults = vec![];
    for (i, item) in Item::ITEMS.iter().copied().enumerate() {
        vaults.push(row.place(
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
        let cur = grid.place(&row, 0, i * 2);
        if let Some(prev) = &prev {
            for v in vaults.iter() {
                grid.connect(v.inside(prev).output(0), v.inside(&cur).input(0));
            }
        }
        prev = Some(cur);
    }
    grid
}

fn stack(y: i32) -> Blueprint {
    let (mana_refinery, mr_w, mr_h, mr_out) = {
        let mut bp = World::new();
        let pump = bp.place(AirPump, 0, 0);
        let refinery = bp.place(Refinery, 2, 0);
        bp.connect(pump.output(0), refinery.input(0));
        (bp, 8, 2, refinery.output(0))
    };

    let mut col = World::new();
    let merge = col.place(BigMerger, 8, y);
    for i in 0..4 {
        let mr = col.place(&mana_refinery, 0, (i as i32) * mr_h);
        col.connect(mr.get(mr_out), merge.input(i + 1))
    }

    let mut bp = World::new();
    let a = bp.place(&col, 0, 0);
    let b = bp.place(&col, 9, 0);
    bp.connect(merge.inside(&a).output(0), merge.inside(&b).input(0));
    let c = bp.place(&col, 18, 0);
    bp.connect(merge.inside(&b).output(0), merge.inside(&c).input(0));

    Blueprint {
        contents: bp,
        size: Size { w: 27, h: 8 },
        inputs: vec![],
        outputs: vec![merge.inside(&c).output(0)],
    }
}

fn main() {
    // let (stack, st_w, st_h, st_outs) = {
    //     let split = bp.place_structure(BigSplitter, 27, 2);
    //     bp.connect(merge_top.inside(&c).output(0), split.input(0));
    //     let disharm_outs: [[PortOut; 4]; 4] = array::from_fn(|i| {
    //         let disharm = bp.place_structure(Disharmonizer, 28 + [0, 4, 4, 0][i], [0, 0, 4, 4][i]);
    //         bp.connect(split.output(i + 1), disharm.input(0));
    //         array::from_fn(|i| disharm.output(i))
    //     });

    //     (bp, 38, 8, disharm_outs)
    // };

    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    world.place(&all_items(), -100, -100);
    world.place(&stack(0), 0, 0);

    // let a = world.paste(&stack, 0, 0);
    // let b = world.paste(&stack, 0, st_h);

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
