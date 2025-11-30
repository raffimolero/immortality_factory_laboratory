use immortality_factory_laboratory::prelude::*;
use std::{
    array,
    fs::File,
    io::{BufWriter, Write},
};

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

fn storage_vault(count: usize, rows: usize, item: Item) -> Blueprint {
    let mut bp = World::new();
    let mut input = None;
    let mut output = None;
    for i in 0..count {
        let cur = bp.place(
            StructureData::StorageVault {
                input: item,
                storage: [item; 16],
                output: item,
            },
            (i / rows) as i32 * StorageVault.width(),
            (i % rows) as i32 * StorageVault.height(),
        );
        if let Some(prev) = output.take() {
            bp.connect(prev, cur.input(0));
        } else {
            input = Some(cur.input(0));
        }
        output = Some(cur.output(0));
    }
    Blueprint {
        contents: bp,
        size: Size {
            w: StorageVault.width(),
            h: StorageVault.height() * count as i32,
        },
        inputs: Vec::from_iter(input),
        outputs: Vec::from_iter(output),
    }
}

/// inputs: [copper] * 4
///
/// outputs: [dust, dust, silica] * 4 + [salt, blood] * 4
fn disharmonizer_stack() -> Blueprint {
    // outputs: [dust, dust, curse, silica] * 4
    let disharm_half = |merge_y| {
        let mut bp = World::new();

        // mana disharmonizers and refinery stacks
        let mut outputs = Vec::with_capacity(16);
        let Size { w, h } = Disharmonizer.size();
        for (i, (dx, dy)) in [(0, 0), (w, 0), (0, h), (w, h)].into_iter().enumerate() {
            let i = i as i32;
            let merge_x = 24 + i;
            let merge = bp.place(BigMerger, merge_x, merge_y);

            let ref_y = i * 2;
            for j in 0..3 {
                let ref_x = j * 8;
                let pump = bp.place(AirPump, ref_x, ref_y);
                let refine = bp.place(Refinery, ref_x + 2, ref_y);
                bp.connect(pump.output(0), refine.input(0));
                bp.connect(refine.output(0), merge.input(j as usize));
            }

            let dh = bp.place(Disharmonizer, 28 + dx, dy);
            bp.connect(merge.output(0), dh.input(0));
            outputs.extend((0..4).map(|i| dh.output(i)));
        }

        Blueprint {
            contents: bp,
            size: Size { w: 36, h: 8 },
            inputs: vec![],
            outputs,
        }
    };
    // inputs: [copper] * 4
    // outputs: [dust, dust, silica] * 4 + [salt, blood] * 4
    let disharm_stack = {
        let mut bp = World::new();
        let mut inputs = vec![];
        let mut outputs = vec![];

        // track the current width of the machine
        let mut bp_w = 0;

        // 2 disharmonizer factories placed so that there are gaps in the middle for mergers
        let top = bp.place(&disharm_half(0), 0, 0);
        let bot = bp.place(&disharm_half(2), 0, top.height());
        bp_w += top.width();
        for dhs in [&top, &bot] {
            for i in 0..4 {
                outputs.push(dhs.output(i * 4 + 0)); // mana dust
                outputs.push(dhs.output(i * 4 + 1)); // mana dust
                outputs.push(dhs.output(i * 4 + 3)); // silica powder
            }
        }

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
        for (i, ((dh_x, dh_y), (uf_x, uf_y))) in [
            ((0, h * 0), (w, 0)),
            ((0, h * 1), (w + 3, 0)),
            ((0, h * 2), (w, h * 3 - 1)),
            ((0, h * 3), (w + 3, h * 3 - 1)),
        ]
        .into_iter()
        .enumerate()
        {
            let dh = bp.place(Disharmonizer, bp_w + dh_x, dh_y);
            let uf = bp.place(Unifier, bp_w + uf_x, uf_y);
            bp.connect(mergers[i].output(0), dh.input(0));
            bp.connect(dh.output(1), uf.input(0));
            bp.connect(dh.output(2), uf.input(1));
            inputs.push(uf.input(2)); // copper coin
            outputs.push(dh.output(0)); // chaos salt
            outputs.push(uf.output(0)); // blood vial
        }
        bp_w += w * 3;

        Blueprint {
            contents: bp,
            size: Size {
                w: bp_w,
                h: top.height() * 2,
            },
            inputs,
            outputs,
        }
    };
    disharm_stack
}

/// inputs: [gold]
///
/// outputs: [3.72x gold]/2s
fn gold_factory() -> Blueprint {
    let gold_factory = {
        let mut bp = World::new();
        let dhs = bp.place(&disharmonizer_stack(), 0, 0);

        // a lot of things
        let glooms: [PortOut; 4] = array::from_fn(|i| {
            let i = i as i32;
            let merge_x = (i % 2) * (Merger.width() * 3) + 40;
            let merge_y = (i / 2) * Merger.height() + 5;
            let merge0 = bp.place(Merger, merge_x, merge_y);
            let merge1 = bp.place(Merger, merge_x + 1, merge_y);
            let merge2 = bp.place(Merger, merge_x + 2, merge_y);
            let ref_x = 46;
            let ref_y = i * 4;
            let ref0 = bp.place(Refinery, ref_x, ref_y);
            let ref1 = bp.place(Refinery, ref_x, ref_y + 2);
            let dhs_silica_idx = i as usize * 6 + 2;
            let dhs_silica_out0 = dhs.output(dhs_silica_idx);
            let dhs_silica_out1 = dhs.output(dhs_silica_idx + 3);
            let dh_gloom = bp.place(
                StructureData::Disharmonizer {
                    input: Empty,
                    outputs: [GloomShard, Empty, Empty, Empty],
                },
                52,
                i * 4,
            );
            bp.connect(dhs_silica_out0, merge0.input(0));
            bp.connect(dhs_silica_out1, merge1.input(0));
            bp.connect(merge0.output(0), ref0.input(0));
            bp.connect(merge1.output(0), ref1.input(0));
            bp.connect(ref0.output(0), merge2.input(0));
            bp.connect(ref1.output(0), merge2.input(1));
            bp.connect(merge2.output(0), dh_gloom.input(0));
            bp.connect(dh_gloom.output(1), merge0.input(1));
            bp.connect(dh_gloom.output(2), merge1.input(1));
            dh_gloom.output(0)
        });

        let merges_coin: [Structure; 6] = array::from_fn(|i| {
            let merge_x = 56 + i as i32;
            let merge_y = 5;
            bp.place(BigMerger, merge_x, merge_y)
        });
        for w in merges_coin.windows(2) {
            bp.connect(w[0].output(0), w[1].input(0));
        }

        // sell dust
        let sells: [Structure; 16] = array::from_fn(|i| {
            let i = i as i32;
            let sell_x = (i / 2) * SubdimensionalMarket.width();
            let sell_y = (i % 2) * SubdimensionalMarket.height() + dhs.height();
            let sell = bp.place(SubdimensionalMarket, sell_x, sell_y);
            let dhs_dust_idx = (i as usize / 2) * 3 + (i as usize % 2);
            bp.connect(dhs.output(dhs_dust_idx), sell.input(0));

            let merge_struct_idx = i as usize / 4;
            let merge_port_idx = i as usize % 4 + 1;
            bp.connect(
                sell.output(0),
                merges_coin[merge_struct_idx].input(merge_port_idx),
            );

            sell
        });

        for i in 0..4 {
            bp.connect(sells[i].output(2), dhs.input(i));
        }

        // sell salt
        for i in 0..2 {
            let sell_x = (i / 2 + 8) * SubdimensionalMarket.width();
            let sell_y = (i % 2) * SubdimensionalMarket.height() + dhs.height();
            let sell = bp.place(SubdimensionalMarket, sell_x, sell_y);
            let dhs_salt_idx = 3 * 8 + (2 * (i as usize * 2 + 1));
            bp.connect(dhs.output(dhs_salt_idx), sell.input(0));

            let merge_struct_idx = 4;
            let merge_port_idx = i as usize % 4 + 1;
            bp.connect(
                sell.output(0),
                merges_coin[merge_struct_idx].input(merge_port_idx),
            );
        }

        // sell blood
        for i in 0..4 {
            let sell_x = (i / 2 + 9) * SubdimensionalMarket.width();
            let sell_y = (i % 2) * SubdimensionalMarket.height() + dhs.height();
            let sell = bp.place(SubdimensionalMarket, sell_x, sell_y);
            let dhs_blood_idx = 3 * 8 + (1) + (i as usize * 2);
            bp.connect(dhs.output(dhs_blood_idx), sell.input(0));

            let merge_struct_idx = 5;
            let merge_port_idx = i as usize % 4 + 1;
            bp.connect(
                sell.output(0),
                merges_coin[merge_struct_idx].input(merge_port_idx),
            );
        }

        // unify and sell orb
        for i in 0..2 {
            // unify
            let uf_x = 56;
            let uf_y = i * 11;
            let uf_bright = bp.place(
                StructureData::Unifier {
                    inputs: [Empty, SilverCoin, Empty],
                    output: BrightShard,
                },
                uf_x,
                uf_y,
            );

            let uf_orb = bp.place(
                StructureData::Unifier {
                    inputs: [GloomShard, BrightShard, Empty],
                    output: Empty,
                },
                uf_x + Unifier.width(),
                uf_y,
            );
            let gloom_idx = i as usize * 2;
            let dhs_salt_idx = 3 * 8 + (2 * (i as usize * 2));
            bp.connect(glooms[gloom_idx + 1], uf_bright.input(0));
            bp.connect(glooms[gloom_idx], uf_orb.input(0));
            bp.connect(uf_bright.output(0), uf_orb.input(1));
            bp.connect(dhs.output(dhs_salt_idx), uf_orb.input(2));

            // sell
            let sell_x = (i / 2 + 11) * SubdimensionalMarket.width();
            let sell_y = (i % 2) * SubdimensionalMarket.height() + dhs.height();
            let sell = bp.place(SubdimensionalMarket, sell_x, sell_y);
            bp.connect(uf_orb.output(0), sell.input(0));
            bp.connect(sell.output(1), uf_bright.input(1));

            let merge_struct_idx = 4;
            let merge_port_idx = i as usize % 4 + 3;
            bp.connect(
                sell.output(0),
                merges_coin[merge_struct_idx].input(merge_port_idx),
            );
        }

        Blueprint {
            contents: bp,
            size: Size {
                w: 62,
                h: dhs.height() + SubdimensionalMarket.height() * 2,
            },
            inputs: vec![merges_coin[0].input(0)],
            outputs: vec![merges_coin[5].output(0)],
        }
    };
    gold_factory
}

/// inputs: [gold, pure]
///
/// outputs: [gold, spark]
fn spark_factory() -> Blueprint {
    let spark_factory = {
        let mut bp = World::new();
        let dhs = bp.place(&disharmonizer_stack(), 0, 0);

        // a lot of things
        let glooms: [PortOut; 4] = array::from_fn(|i| {
            let i = i as i32;
            let merge_x = (i % 2) * (Merger.width() * 3) + 40;
            let merge_y = (i / 2) * Merger.height() + 5;
            let merge0 = bp.place(Merger, merge_x, merge_y);
            let merge1 = bp.place(Merger, merge_x + 1, merge_y);
            let merge2 = bp.place(Merger, merge_x + 2, merge_y);
            let ref_x = 46;
            let ref_y = i * 4;
            let ref0 = bp.place(Refinery, ref_x, ref_y);
            let ref1 = bp.place(Refinery, ref_x, ref_y + 2);
            let dhs_silica_idx = i as usize * 6 + 2;
            let dhs_silica_out0 = dhs.output(dhs_silica_idx);
            let dhs_silica_out1 = dhs.output(dhs_silica_idx + 3);
            let dh_gloom = bp.place(
                StructureData::Disharmonizer {
                    input: ObsidianPlate,
                    outputs: [GloomShard, SilicaPowder, SilicaPowder, Empty],
                },
                52,
                i * 4,
            );
            bp.connect(dhs_silica_out0, merge0.input(0));
            bp.connect(dhs_silica_out1, merge1.input(0));
            bp.connect(merge0.output(0), ref0.input(0));
            bp.connect(merge1.output(0), ref1.input(0));
            bp.connect(ref0.output(0), merge2.input(0));
            bp.connect(ref1.output(0), merge2.input(1));
            bp.connect(merge2.output(0), dh_gloom.input(0));
            bp.connect(dh_gloom.output(1), merge0.input(1));
            bp.connect(dh_gloom.output(2), merge1.input(1));
            dh_gloom.output(0)
        });

        let merges_coin: [Structure; 5] = array::from_fn(|i| {
            let merge_x = 56 + i as i32;
            let merge_y = 5;
            bp.place(BigMerger, merge_x, merge_y)
        });
        for w in merges_coin[0..3].windows(2) {
            bp.connect(w[0].output(0), w[1].input(0));
        }
        let split_pure = bp.place(Splitter, 61, 5);
        let merge_spark = bp.place(Merger, 61, 8);

        // sell dust
        let sells: [Structure; 16] = array::from_fn(|i| {
            let i = i as i32;
            let sell_x = (i / 2) * SubdimensionalMarket.width();
            let sell_y = (i % 2) * SubdimensionalMarket.height() + dhs.height();
            let sell = bp.place(SubdimensionalMarket, sell_x, sell_y);
            let dhs_dust_idx = (i as usize / 2) * 3 + (i as usize % 2);
            bp.connect(dhs.output(dhs_dust_idx), sell.input(0));

            if i < 6 {
                bp.connect(
                    sell.output(1),
                    merges_coin[3 + (i as usize % 2)].input(4 - (i as usize / 2)),
                );
            } else {
                let i = i - 6;
                let merge_struct_idx = i as usize / 4;
                let merge_port_idx = i as usize % 4 + 1;
                bp.connect(
                    sell.output(0),
                    merges_coin[merge_struct_idx].input(merge_port_idx),
                );
            }

            sell
        });

        for i in 0..4 {
            bp.connect(sells[i].output(2), dhs.input(i));
        }

        // unify orb and spark
        for i in 0..2 {
            // unify orb
            let orb = {
                let uf_x = 56;
                let uf_y = i * 11;
                let uf_bright = bp.place(
                    StructureData::Unifier {
                        inputs: [Empty, SilverCoin, Empty],
                        output: BrightShard,
                    },
                    uf_x,
                    uf_y,
                );

                let uf_orb = bp.place(
                    StructureData::Unifier {
                        inputs: [GloomShard, BrightShard, Empty],
                        output: Empty,
                    },
                    uf_x + Unifier.width(),
                    uf_y,
                );
                let gloom_idx = i as usize * 2;
                let dhs_salt_idx = 3 * 8 + (2 * (i as usize * 2));
                bp.connect(glooms[gloom_idx + 1], uf_bright.input(0));
                bp.connect(merges_coin[3 + i as usize].output(0), uf_bright.input(1));
                bp.connect(glooms[gloom_idx], uf_orb.input(0));
                bp.connect(uf_bright.output(0), uf_orb.input(1));
                bp.connect(dhs.output(dhs_salt_idx), uf_orb.input(2));
                uf_orb.output(0)
            };

            // create life force
            let life = {
                let i = i as i32;
                let uf_x = (i / 2 + 8) * SubdimensionalMarket.width();
                let uf_y = (i % 2) * Unifier.height() + dhs.height();
                let uf_life = bp.place(
                    StructureData::Unifier {
                        inputs: [VialOfBlood, VialOfBlood, Empty],
                        output: Empty,
                    },
                    uf_x,
                    uf_y,
                );
                let dhs_salt_idx = 3 * 8 + (2 * (i as usize * 2 + 1));
                let dhs_blood_idx_0 = 3 * 8 + (1) + (i as usize * 4);
                let dhs_blood_idx_1 = 3 * 8 + (1) + (i as usize * 4 + 2);
                bp.connect(dhs.output(dhs_blood_idx_0), uf_life.input(0));
                bp.connect(dhs.output(dhs_blood_idx_1), uf_life.input(1));
                bp.connect(dhs.output(dhs_salt_idx), uf_life.input(2));
                uf_life.output(0)
            };

            let (pure, spark) = {
                let uf_x = (i / 2 + 8) * SubdimensionalMarket.width() + Unifier.width();
                let uf_y = (i % 2) * Unifier.height() + dhs.height();
                let uf_spark = bp.place(Unifier, uf_x, uf_y);
                bp.connect(orb, uf_spark.input(1));
                bp.connect(life, uf_spark.input(2));
                (uf_spark.input(0), uf_spark.output(0))
            };
            bp.connect(split_pure.output(i as usize), pure);
            bp.connect(spark, merge_spark.input(i as usize));
        }

        Blueprint {
            contents: bp,
            size: Size {
                w: 62,
                h: dhs.height() + SubdimensionalMarket.height() * 2,
            },
            inputs: vec![merges_coin[0].input(0), split_pure.input(0)],
            outputs: vec![merges_coin[2].output(0), merge_spark.output(0)],
        }
    };
    spark_factory
}
fn stuff() -> World {
    let mut world = World::new();
    let pure_vault_count = 16;
    let pure_vault = world.place(
        &storage_vault(pure_vault_count, pure_vault_count, PureManaGem),
        -StorageVault.width(),
        0,
    );
    let sf = world.place(&spark_factory(), 0, 0);
    world.connect(pure_vault.output(0), sf.input(1));
    {
        let sv_gold = world.place(&storage_vault(8 * 4, 4, Empty), 0, sf.height());
        world.connect(sf.output(0), sv_gold.input(0));
    }
    {
        let sv_spark = world.place(
            &storage_vault(8 * 4, 4, Empty),
            0,
            sf.height() + StorageVault.height() * 4,
        );
        world.connect(sf.output(1), sv_spark.input(0));
    }
    world
}

fn main() {
    let mut world = World::new();
    world.place(Laboratory, 0, -2);
    world.place(&all_items(), -100, -100);
    world.place(&stuff(), 0, 0);

    let file = File::create("../save.ini").expect("Failed to create file.");
    let mut writer = BufWriter::new(file);
    world
        .export(&mut writer)
        .and_then(|()| writer.flush()) // docs told me to flush it manually instead of relying on Drop
        .expect("Failed to write to file.");
}
