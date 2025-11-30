use crate::prelude::*;

use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectorData {
    pub inputs: &'static [Offset],
    pub outputs: &'static [Offset],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureData {
    AirPump {
        output: Item,
    },
    Refinery {
        input: Item,
        storage: [Item; 12],
        output: Item,
    },
    Disharmonizer {
        input: Item,
        outputs: [Item; 4],
    },
    Unifier {
        inputs: [Item; 3],
        output: Item,
    },
    SubdimensionalMarket {
        input: Item,
        outputs: [Item; 3],
    },
    Splitter {
        input: Item,
        outputs: [Item; 2],
    },
    Merger {
        inputs: [Item; 2],
        output: Item,
    },
    StorageVault {
        input: Item,
        storage: [Item; 16],
        output: Item,
    },
    AbysalDoor {
        input: Item,
    },
    SingleStorage {
        // technically considered an output in the code
        output: Item,
    },
    Laboratory {
        input: Item,
    },
    RitualInfuser {
        inputs: [Item; 3],
        output: Item,
    },
    BigMerger {
        inputs: [Item; 5],
        output: Item,
    },
    BigSplitter {
        input: Item,
        outputs: [Item; 5],
    },
}

impl StructureData {
    pub fn kind(&self) -> StructureKind {
        self.into()
    }

    pub fn get_storage(&self) -> &[Item] {
        match self {
            Self::Refinery { storage, .. } => storage,
            Self::StorageVault { storage, .. } => storage,
            _ => &[],
        }
    }

    fn export_stored_item(
        f: &mut impl Write,
        id: usize,
        item_index: usize,
        item: Item,
    ) -> io::Result<()> {
        let item_id = item as i8;
        writeln!(f, "{id}-storage_load_at {item_index}=\"{item_id}.000000\"")
    }

    pub fn export(&self, f: &mut impl Write, id: usize, raw_x: i32, raw_y: i32) -> io::Result<()> {
        let world_y = raw_y * 22;
        let world_x = raw_x * 22;
        let obj_num = self.kind().object_number();

        self.export_struct(f, id)?;
        writeln!(f, "{id}-y=\"{world_y}.000000\"")?;
        writeln!(f, "{id}-x=\"{world_x}.000000\"")?;
        writeln!(f, "{id}-object=\"{obj_num}.000000\"")?;
        for (i, item) in self.get_storage().iter().enumerate() {
            Self::export_stored_item(f, id, i, *item)?;
        }
        Ok(())
    }

    fn export_struct(&self, f: &mut impl Write, id: usize) -> io::Result<()> {
        match *self {
            Self::AirPump { output } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:1.0,+row+:1.0,+content_column+:1.0,+type+:1,+content_row+:0.0,+content+:{}.0}}],+type+:0.0,+machine_type+:{{+name+:+Air Pump+,+type+:0,+description+:+Sucks in potent air from the surrounding valley and puts it in a bottle.+,+sprite+:5,+machine_cost+:{{+cost_type_list+:[8,0,0,1,1,2,2,5,15,16,16,16,7,7,7,7,7,7,20,20,20,21,21,21,21,21,21],+cost_amount_list+:[3.0,2.0,4.0,4.0,4.0,4.0,3.0,4.0,5.0,3.0,3.0,3.0,4.0,4.0,3.0,3.0,2.0,2.0,3.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:8.0}},+input_list+:[]}}""#,
                output as i8,
            ),
            Self::Refinery {
                input,
                storage: _,
                output,
            } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:5.0,+row+:0.0,+content_column+:5.0,+type+:1,+content_row+:1.0,+content+:{}.0}}],+type+:1.0,+machine_type+:{{+name+:+Refinery+,+type+:1,+description+:+Improves a resource, turning it into something better.+,+sprite+:35,+machine_cost+:{{+cost_type_list+:[0,1,1,1,1,3,3,3,3,3,3,3,15,15,15,15,16,16,16],+cost_amount_list+:[3.0,2.0,2.0,2.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:0.0,+type+:0,+content_row+:1.0,+content+:{}.0}}]}}""#,
                output as i8, input as i8,
            ),
            Self::Disharmonizer { input, outputs } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:3.0,+row+:0.0,+content_column+:2.0,+type+:1,+content_row+:0.0,+content+:{}.0}},{{+index+:1.0,+column+:3.0,+row+:1.0,+content_column+:2.0,+type+:1,+content_row+:1.0,+content+:{}.0}},{{+index+:2.0,+column+:3.0,+row+:2.0,+content_column+:2.0,+type+:1,+content_row+:2.0,+content+:{}.0}},{{+index+:3.0,+column+:3.0,+row+:3.0,+content_column+:2.0,+type+:1,+content_row+:3.0,+content+:{}.0}}],+type+:2.0,+machine_type+:{{+name+:+Disharmonizer+,+type+:2,+description+:+Breaks resources apart by nature and magical sequence.+,+sprite+:37,+machine_cost+:{{+cost_type_list+:[1,5,5,15,17,17,17,17,17,17,17,17,18,18,18,18,18,18,20,20,20,20,20,21,21,21,21],+cost_amount_list+:[3.0,4.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,3.0,2.0,2.0,4.0,3.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:3.0,+content_column+:0.0,+type+:0,+content_row+:2.0,+content+:{}.0}}]}}""#,
                outputs[0] as i8, outputs[1] as i8, outputs[2] as i8, outputs[3] as i8, input as i8,
            ),
            Self::Unifier { inputs, output } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:1.0,+row+:0.0,+content_column+:1.0,+type+:1,+content_row+:1.0,+content+:{}.0}}],+type+:3.0,+machine_type+:{{+name+:+Unifier+,+type+:3,+description+:+Converges multiple resources into one.+,+sprite+:61,+machine_cost+:{{+cost_type_list+:[2,15,15,15,15,15,7,7,7,16,16,16,16],+cost_amount_list+:[4.0,4.0,3.0,2.0,2.0,2.0,3.0,3.0,2.0,4.0,3.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:4.0,+content_column+:0.0,+type+:0,+content_row+:3.0,+content+:{}.0}},{{+index+:1.0,+column+:1.0,+row+:4.0,+content_column+:1.0,+type+:0,+content_row+:3.0,+content+:{}.0}},{{+index+:2.0,+column+:2.0,+row+:4.0,+content_column+:2.0,+type+:0,+content_row+:3.0,+content+:{}.0}}]}}""#,
                output as i8, inputs[0] as i8, inputs[1] as i8, inputs[2] as i8,
            ),
            Self::SubdimensionalMarket { input, outputs } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:3.0,+row+:0.0,+content_column+:2.0,+type+:1.0,+content_row+:0.0,+content+:{}.0}},{{+index+:1.0,+column+:3.0,+row+:1.0,+content_column+:2.0,+type+:1.0,+content_row+:1.0,+content+:{}}},{{+index+:2.0,+column+:3.0,+row+:2.0,+content_column+:2.0,+type+:1.0,+content_row+:2.0,+content+:{}}}],+type+:4.0,+machine_type+:{{+name+:+Subdimensional Market+,+type+:4.0,+description+:+Sell any resource for coin. Some are more worth than others.+,+sprite+:52.0,+machine_cost+:{{+cost_type_list+:[10.0,11.0,11.0,8.0,8.0,21.0],+cost_amount_list+:[4.0,4.0,3.0,4.0,3.0,2.0]}},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:1.0,+machine_speed+:8.0}},+input_list+:[{{+index+:0.0,+column+:3.0,+row+:4.0,+content_column+:2.0,+type+:0.0,+content_row+:4.0,+content+:{}.0}}]}}""#,
                outputs[0] as i8, outputs[1] as i8, outputs[2] as i8, input as i8,
            ),
            Self::Splitter { input, outputs } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:1.0,+content_row+:-1.0,+content+:{}.0}},{{+index+:1.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:1.0,+content_row+:-1.0,+content+:{}.0}}],+type+:5.0,+machine_type+:{{+name+:+Splitter+,+type+:5.0,+description+:+Split an incomming connection into two outputs.+,+sprite+:24.0,+machine_cost+:{{+cost_type_list+:[1.0,1.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,3.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:0.0,+content_row+:-1.0,+content+:{}.0}}]}}""#,
                outputs[0] as i8, outputs[1] as i8, input as i8,
            ),
            Self::Merger { inputs, output } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:1.0,+content_row+:-1.0,+content+:{}.0}}],+type+:6.0,+machine_type+:{{+name+:+Merger+,+type+:6.0,+description+:+Merges two incomming connections into one output.+,+sprite+:25.0,+machine_cost+:{{+cost_type_list+:[1.0,1.0,1.0,1.0,4.0,4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,2.0,2.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:0.0,+content_row+:-1.0,+content+:{}.0}},{{+index+:1.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:0.0,+content_row+:-1.0,+content+:{}.0}}]}}""#,
                output as i8, inputs[0] as i8, inputs[1] as i8,
            ),
            Self::StorageVault {
                input,
                storage: _,
                output,
            } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:4.0,+row+:1.0,+content_column+:4.0,+type+:1.0,+content_row+:0.0,+content+:{}.0}}],+type+:7.0,+machine_type+:{{+name+:+Storage Vault+,+type+:7.0,+description+:+A machine which keeps your resources safe behind thick glass.+,+sprite+:6.0,+machine_cost+:{{+cost_type_list+:[4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:0.0,+type+:0.0,+content_row+:0.0,+content+:{}.0}}]}}""#,
                output as i8, input as i8,
            ),
            Self::AbysalDoor { input } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[],+type+:8.0,+machine_type+:{{+name+:+Abysal Door+,+type+:8.0,+description+:+Get rid of all you don't have a need for.+,+sprite+:3.0,+machine_cost+:{{+cost_type_list+:[2.0,2.0,2.0,2.0,2.0],+cost_amount_list+:[4.0,3.0,3.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:1.0,+machine_speed+:2.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:1.0,+type+:0.0,+content_row+:0.0,+content+:{}.0}}]}}""#,
                input as i8,
            ),
            Self::SingleStorage {
                // technically considered an output in the code
                output,
            } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[],+type+:9.0,+machine_type+:{{+name+:+Single Storage+,+type+:9.0,+description+:+A single storage place for a single resource.+,+sprite+:17.0,+machine_cost+:{{+cost_type_list+:[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,5.0,5.0,5.0],+cost_amount_list+:[2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:-1.0,+row+:-1.0,+content_column+:0.0,+type+:0.0,+content_row+:0.0,+content+:{}.0}}]}}""#,
                output as i8,
            ),
            Self::Laboratory { input } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[],+type+:10.0,+machine_type+:{{+name+:+Laboratory+,+type+:10.0,+description+:+Used to research more stuff.+,+sprite+:45.0,+machine_cost+:{{+cost_type_list+:[8.0],+cost_amount_list+:[100.0]}},+cost_input+:10.0,+speed_increase+:32.0,+unlocked+:0.0,+machine_speed+:4.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:0.0,+type+:0.0,+content_row+:0.0,+content+:{}.0}}]}}""#,
                input as i8
            ),
            Self::RitualInfuser { inputs, output } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:-1.0,+row+:-1.0,+content_column+:2.0,+type+:1.0,+content_row+:3.0,+content+:{}.0}}],+type+:11.0,+machine_type+:{{+name+:+Ritual Infuser+,+type+:11.0,+description+:+Automate magical rituals. Used to create the phylactery.+,+sprite+:44.0,+machine_cost+:{{+cost_type_list+:[21.0,25.0,25.0,25.0,25.0],+cost_amount_list+:[8.0,2.0,2.0,1.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:1.0,+type+:0.0,+content_row+:1.0,+content+:{}.0}},{{+index+:1.0,+column+:2.0,+row+:0.0,+content_column+:2.0,+type+:0.0,+content_row+:1.0,+content+:{}.0}},{{+index+:2.0,+column+:4.0,+row+:1.0,+content_column+:3.0,+type+:0.0,+content_row+:1.0,+content+:{}.0}}]}}""#,
                output as i8, inputs[0] as i8, inputs[1] as i8, inputs[2] as i8,
            ),
            Self::BigMerger { inputs, output } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:5.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:{}.0}}],+type+:12,+machine_type+:{{+name+:+Big Merger+,+type+:12,+description+:+Merges Inputs. Lowest always first.+,+sprite+:53,+machine_cost+:{{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-10}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:{}.0}},{{+index+:1.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:{}.0}},{{+index+:2.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:{}.0}},{{+index+:3.0,+column+:0.0,+row+:3.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:{}.0}},{{+index+:4.0,+column+:0.0,+row+:4.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:{}.0}}]}}""#,
                output as i8,
                inputs[0] as i8,
                inputs[1] as i8,
                inputs[2] as i8,
                inputs[3] as i8,
                inputs[4] as i8,
            ),
            Self::BigSplitter { input, outputs } => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:{}.0}},{{+index+:1.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:{}.0}},{{+index+:2.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:{}.0}},{{+index+:3.0,+column+:0.0,+row+:3.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:{}.0}},{{+index+:4.0,+column+:0.0,+row+:4.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:{}.0}}],+type+:13,+machine_type+:{{+name+:+Big Splitter+,+type+:13,+description+:+Splits Outputs. Lowest always first.+,+sprite+:22,+machine_cost+:{{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-10}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:5.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:{}.0}}]}}""#,
                outputs[0] as i8,
                outputs[1] as i8,
                outputs[2] as i8,
                outputs[3] as i8,
                outputs[4] as i8,
                input as i8,
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureKind {
    AirPump,
    Refinery,
    Disharmonizer,
    Unifier,
    SubdimensionalMarket,
    Splitter,
    Merger,
    StorageVault,
    AbysalDoor, // [sic]
    SingleStorage,
    Laboratory,
    RitualInfuser,
    BigMerger,
    BigSplitter,
}

impl StructureKind {
    /// i have no idea what this number means
    fn object_number(&self) -> u8 {
        match self {
            Self::AirPump => 3,
            Self::Refinery => 5,
            Self::Disharmonizer => 2,
            Self::Unifier => 1,
            Self::SubdimensionalMarket => 6,
            Self::Splitter => 23,
            Self::Merger => 26,
            Self::StorageVault => 0,
            Self::AbysalDoor => 18,
            Self::SingleStorage => 19,
            Self::Laboratory => 20,
            Self::RitualInfuser => 12,
            Self::BigMerger => 4,
            Self::BigSplitter => 9,
        }
    }

    pub fn connectors(&self) -> ConnectorData {
        match self {
            Self::AirPump => ConnectorData {
                inputs: &[],
                outputs: &[Offset { x: 1, y: 1 }],
            },
            Self::Refinery => ConnectorData {
                inputs: &[Offset { x: 0, y: 0 }],
                outputs: &[Offset { x: 5, y: 0 }],
            },
            Self::Disharmonizer => ConnectorData {
                inputs: &[Offset { x: 0, y: 3 }],
                outputs: &[
                    Offset { x: 3, y: 0 },
                    Offset { x: 3, y: 1 },
                    Offset { x: 3, y: 2 },
                    Offset { x: 3, y: 3 },
                ],
            },
            Self::Unifier => ConnectorData {
                inputs: &[
                    Offset { x: 0, y: 4 },
                    Offset { x: 1, y: 4 },
                    Offset { x: 2, y: 4 },
                ],
                outputs: &[Offset { x: 1, y: 0 }],
            },
            Self::SubdimensionalMarket => ConnectorData {
                inputs: &[Offset { x: 3, y: 4 }],
                outputs: &[
                    Offset { x: 3, y: 0 },
                    Offset { x: 3, y: 1 },
                    Offset { x: 3, y: 2 },
                ],
            },
            Self::Splitter => ConnectorData {
                inputs: &[Offset { x: 0, y: 1 }],
                outputs: &[Offset { x: 0, y: 0 }, Offset { x: 0, y: 2 }],
            },
            Self::Merger => ConnectorData {
                inputs: &[Offset { x: 0, y: 0 }, Offset { x: 0, y: 2 }],
                outputs: &[Offset { x: 0, y: 1 }],
            },
            Self::StorageVault => ConnectorData {
                inputs: &[Offset { x: 0, y: 1 }],
                outputs: &[Offset { x: 4, y: 1 }],
            },
            Self::AbysalDoor => ConnectorData {
                inputs: &[Offset { x: 0, y: 0 }],
                outputs: &[],
            },
            Self::SingleStorage => ConnectorData {
                inputs: &[],
                outputs: &[Offset::NULL],
            },
            Self::Laboratory => ConnectorData {
                inputs: &[Offset { x: 0, y: 1 }],
                outputs: &[],
            },
            Self::RitualInfuser => ConnectorData {
                inputs: &[
                    Offset { x: 0, y: 1 },
                    Offset { x: 2, y: 0 },
                    Offset { x: 4, y: 1 },
                ],
                outputs: &[Offset::NULL],
            },
            Self::BigMerger => ConnectorData {
                inputs: &[
                    Offset { x: 0, y: 0 },
                    Offset { x: 0, y: 1 },
                    Offset { x: 0, y: 2 },
                    Offset { x: 0, y: 3 },
                    Offset { x: 0, y: 4 },
                ],
                outputs: &[Offset { x: 0, y: 5 }],
            },
            Self::BigSplitter => ConnectorData {
                inputs: &[Offset { x: 0, y: 5 }],
                outputs: &[
                    Offset { x: 0, y: 0 },
                    Offset { x: 0, y: 1 },
                    Offset { x: 0, y: 2 },
                    Offset { x: 0, y: 3 },
                    Offset { x: 0, y: 4 },
                ],
            },
        }
    }
}

impl From<&StructureData> for StructureKind {
    fn from(value: &StructureData) -> Self {
        match value {
            StructureData::AirPump { .. } => Self::AirPump,
            StructureData::Refinery { .. } => Self::Refinery,
            StructureData::Disharmonizer { .. } => Self::Disharmonizer,
            StructureData::Unifier { .. } => Self::Unifier,
            StructureData::SubdimensionalMarket { .. } => Self::SubdimensionalMarket,
            StructureData::Splitter { .. } => Self::Splitter,
            StructureData::Merger { .. } => Self::Merger,
            StructureData::StorageVault { .. } => Self::StorageVault,
            StructureData::AbysalDoor { .. } => Self::AbysalDoor,
            StructureData::SingleStorage { .. } => Self::SingleStorage,
            StructureData::Laboratory { .. } => Self::Laboratory,
            StructureData::RitualInfuser { .. } => Self::RitualInfuser,
            StructureData::BigMerger { .. } => Self::BigMerger,
            StructureData::BigSplitter { .. } => Self::BigSplitter,
        }
    }
}

impl From<StructureKind> for StructureData {
    fn from(value: StructureKind) -> Self {
        use Item::Empty;
        match value {
            StructureKind::AirPump => Self::AirPump { output: Empty },
            StructureKind::Refinery => Self::Refinery {
                input: Empty,
                storage: [Empty; 12],
                output: Empty,
            },
            StructureKind::Disharmonizer => Self::Disharmonizer {
                input: Empty,
                outputs: [Empty; 4],
            },
            StructureKind::Unifier => Self::Unifier {
                inputs: [Empty; 3],
                output: Empty,
            },
            StructureKind::SubdimensionalMarket => Self::SubdimensionalMarket {
                input: Empty,
                outputs: [Empty; 3],
            },
            StructureKind::Splitter => Self::Splitter {
                input: Empty,
                outputs: [Empty; 2],
            },
            StructureKind::Merger => Self::Merger {
                inputs: [Empty; 2],
                output: Empty,
            },
            StructureKind::StorageVault => Self::StorageVault {
                input: Empty,
                storage: [Empty; 16],
                output: Empty,
            },
            StructureKind::AbysalDoor => Self::AbysalDoor { input: Empty },
            StructureKind::SingleStorage => Self::SingleStorage { output: Empty },
            StructureKind::Laboratory => Self::Laboratory { input: Empty },
            StructureKind::RitualInfuser => Self::RitualInfuser {
                inputs: [Empty; 3],
                output: Empty,
            },
            StructureKind::BigMerger => Self::BigMerger {
                inputs: [Empty; 5],
                output: Empty,
            },
            StructureKind::BigSplitter => Self::BigSplitter {
                input: Empty,
                outputs: [Empty; 5],
            },
        }
    }
}

impl HasSize for StructureKind {
    fn size(&self) -> Size {
        match self {
            Self::AirPump => Size { w: 2, h: 2 },
            Self::Refinery => Size { w: 6, h: 2 },
            Self::Disharmonizer => Size { w: 4, h: 4 },
            Self::Unifier => Size { w: 3, h: 5 },
            Self::SubdimensionalMarket => Size { w: 4, h: 5 },
            Self::Splitter => Size { w: 1, h: 3 },
            Self::Merger => Size { w: 1, h: 3 },
            Self::StorageVault => Size { w: 5, h: 2 },
            Self::AbysalDoor => Size { w: 4, h: 1 },
            Self::SingleStorage => Size { w: 1, h: 1 },
            Self::Laboratory => Size { w: 5, h: 2 },
            Self::RitualInfuser => Size { w: 5, h: 5 },
            Self::BigMerger => Size { w: 1, h: 6 },
            Self::BigSplitter => Size { w: 1, h: 6 },
        }
    }
}
