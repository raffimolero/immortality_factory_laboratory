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

fn mana_stack(merge_y: i32) -> Blueprint {
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
        let merge = bp.place(BigMerger, 8, merge_y);
        for i in 0..4 {
            let mr = bp.place(&mana_refinery, 0, (i as i32) * mana_refinery.height());
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
        let c = bp.place(&refinery_column, 18, 0);
        bp.connect(a.output(0), b.input(0));
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

fn disharmonizer_stack() -> Blueprint {
    let disharm_half = |merge_y| {
        let mut bp = World::new();
        let mut x = 0;
        // refinery stacks
        let mana = bp.place(&mana_stack(merge_y), x, 0);
        x += mana.width();

        // splitter
        let split = bp.place(BigSplitter, x, merge_y);
        bp.connect(mana.output(0), split.input(0));
        x += split.width();

        let mut outputs = Vec::with_capacity(16);
        let Size { w, h } = Disharmonizer.size();
        for (i, (dx, dy)) in [(0, 0), (w, 0), (0, h), (w, h)].into_iter().enumerate() {
            let dh = bp.place(Disharmonizer, x + dx, dy);
            bp.connect(split.output(i + 1), dh.input(0));
            outputs.extend((0..4).map(|i| dh.output(i)));
        }
        x += w * 2;
        Blueprint {
            contents: bp,
            size: Size { w: x, h: 8 },
            inputs: vec![],
            outputs,
        }
    };
    let disharm_stack = {
        let mut bp = World::new();

        // 2 disharmonizer factories placed so that there are gaps in the middle for mergers
        let top = bp.place(&disharm_half(0), 0, 0);
        let bot = bp.place(&disharm_half(2), 0, top.height());

        // place mergers in empty space
        let mergers = [8, 17, 26, 27].map(|x| bp.place(Merger, x, 6));

        // wire disharmonizers to mergers in a pattern
        // needs a bunch of math to compute
        for i in 0..16 {
            let merger = mergers[i / 4];
            let merger_input = i % 2;
            let half = [&top, &bot][i / 8];
            let output_per_disharm = 4;
            let curse_slot = 2;
            let disharm_output = (i % 4) * output_per_disharm + curse_slot;
            bp.connect(half.output(disharm_output), merger.input(merger_input));
        }

        Blueprint {
            contents: bp,
            size: Size {
                w: top.width(),
                h: top.height() * 2,
            },
            inputs: vec![],
            outputs: vec![],
        }
    };
    disharm_stack
}

fn stuff() -> World {
    let mut world = World::new();
    // let top = world.place(&mana_stack(0), 0, 0);
    // let bot = world.place(&mana_stack(2), 0, 8);
    // let merger = world.place(BigMerger, top.width(), 0);
    // world.connect(top.output(0), merger.input(0));
    // world.connect(bot.output(0), merger.input(1));
    let st = disharmonizer_stack();
    world.place(&st, 0, 0);
    world
}

fn main() {
    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    world.place(&all_items(), -100, -100);
    world.place(&stuff(), 0, 0);

    // let a = world.paste(&stack, 0, 0);
    // let b = world.paste(&stack, 0, st_h);

    let mut out = String::new();
    world.export(&mut out).expect("write failed");
    fs::write("../save.ini", out).expect("Failed to create file.");
}
