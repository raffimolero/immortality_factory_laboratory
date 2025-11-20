mod immortality_factory;

use immortality_factory::prelude::*;
use std::fs;

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
    let mana_refinery = {
        let mut bp = World::new();
        let pump = bp.place(AirPump, 0, 0);
        let refinery = bp.place(Refinery, 2, 0);
        bp.connect(pump.output(0), refinery.input(0));
        Blueprint {
            contents: bp,
            size: Size { w: 8, h: 2 },
            inputs: vec![],
            outputs: vec![refinery.output(0)],
        }
    };

    let refinery_column = {
        let mut bp = World::new();
        let merge = bp.place(BigMerger, 8, y);
        for i in 0..4 {
            let mr = bp.place(&mana_refinery, 0, (i as i32) * mana_refinery.h());
            bp.connect(mr.output(0), merge.input(i + 1))
        }
        Blueprint {
            contents: bp,
            size: Size { w: 9, h: 8 },
            inputs: vec![merge.input(0)],
            outputs: vec![merge.output(0)],
        }
    };

    let refinery_stack = {
        let mut bp = World::new();
        let a = bp.place(&refinery_column, 0, 0);
        let b = bp.place(&refinery_column, 9, 0);
        bp.connect(a.output(0), b.input(0));
        let c = bp.place(&refinery_column, 18, 0);
        bp.connect(b.output(0), c.input(0));

        Blueprint {
            contents: bp,
            size: Size { w: 27, h: 8 },
            inputs: vec![],
            outputs: vec![c.output(0)],
        }
    };
    refinery_stack
}

fn main() {
    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    world.place(&all_items(), -100, -100);

    let top = world.place(&stack(0), 0, 0);
    let bot = world.place(&stack(2), 0, 8);
    let merger = world.place(BigMerger, top.w(), 0);
    world.connect(top.output(0), merger.input(0));
    world.connect(bot.output(0), merger.input(1));

    // let a = world.paste(&stack, 0, 0);
    // let b = world.paste(&stack, 0, st_h);

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
