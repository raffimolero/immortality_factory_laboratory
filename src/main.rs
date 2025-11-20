mod immortality_factory;

use immortality_factory::prelude::*;
use std::fs;

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

    let refinery_stack = {
        let mut bp = World::new();
        let mr_w = mana_refinery.width();
        let mr_h = mana_refinery.height();

        // make grid
        let mut prev = None::<Structure>;
        for grid_x in 0..3 {
            let mr_x = grid_x as i32 * mr_w;
            let merge = bp.place(BigMerger, mr_w * 3 + grid_x, merge_y);
            for grid_y in 0..4 {
                let mr_y = grid_y as i32 * mr_h;
                let mr = bp.place(&mana_refinery, mr_x, mr_y);
                bp.connect(mr.output(0), merge.input(grid_y + 1))
            }
            if let Some(prev) = prev {
                bp.connect(prev.output(0), merge.input(0));
            }
            prev = Some(merge);
        }
        Blueprint {
            contents: bp,
            size: Size { w: 27, h: 8 },
            inputs: vec![],
            outputs: vec![prev.unwrap().output(0)],
        }
    };
    refinery_stack
}

fn disharmonizer_stack() -> Blueprint {
    let disharm_half = |merge_y| {
        let mut bp = World::new();
        let mut bp_w = 0;
        // refinery stacks
        let mana = bp.place(&mana_stack(merge_y), bp_w, 0);
        bp_w += mana.width();

        // splitter
        let split = bp.place(BigSplitter, bp_w, merge_y);
        bp.connect(mana.output(0), split.input(0));
        bp_w += split.width();

        // mana disharmonizers
        let mut outputs = Vec::with_capacity(16);
        let Size { w, h } = Disharmonizer.size();
        for (i, (dx, dy)) in [(0, 0), (w, 0), (0, h), (w, h)].into_iter().enumerate() {
            let dh = bp.place(Disharmonizer, bp_w + dx, dy);
            bp.connect(split.output(i + 1), dh.input(0));
            outputs.extend((0..4).map(|i| dh.output(i)));
        }
        bp_w += w * 2;

        Blueprint {
            contents: bp,
            size: Size { w: bp_w, h: 8 },
            inputs: vec![],
            outputs,
        }
    };
    let disharm_stack = {
        let mut bp = World::new();
        // track the current width of the machine
        let mut bp_w = 0;

        // 2 disharmonizer factories placed so that there are gaps in the middle for mergers
        let top = bp.place(&disharm_half(0), 0, 0);
        let bot = bp.place(&disharm_half(2), 0, top.height());
        bp_w += top.width();

        // place mergers in empty space
        let mergers = [24, 25, 26, 27].map(|x| bp.place(Merger, x, 6));
        // does not increase width

        // wire disharmonizers to mergers in a pattern
        // needs a bunch of math to compute
        for i in 0..8 {
            let merger = mergers[i / 2];
            let merger_input = i % 2;
            let half = [&top, &bot][i / 4];
            let output_per_disharm = 4;
            let curse_slot = 2;
            let disharm_output = (i % 4) * output_per_disharm + curse_slot;
            bp.connect(half.output(disharm_output), merger.input(merger_input));
        }

        // make curse disharmonizers and blood unifiers
        let Size { w, h } = Disharmonizer.size();
        let mut i = 0;
        let c_dh = [
            ((0, h * 0), (w, 0)),
            ((0, h * 1), (w + 3, 0)),
            ((0, h * 2), (w, h * 3 - 1)),
            ((0, h * 3), (w + 3, h * 3 - 1)),
        ]
        .map(|((dh_x, dh_y), (uf_x, uf_y))| {
            let dh = bp.place(Disharmonizer, bp_w + dh_x, dh_y);
            let uf = bp.place(Unifier, bp_w + uf_x, uf_y);
            bp.connect(mergers[i].output(0), dh.input(0));
            bp.connect(dh.output(1), uf.input(0));
            bp.connect(dh.output(2), uf.input(1));
            i += 1;
        });
        bp_w += w * 3;

        Blueprint {
            contents: bp,
            size: Size {
                w: bp_w,
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
