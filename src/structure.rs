use crate::{item::Item, world::Offset};
use std::{
    fmt::{self, Write},
    mem::discriminant,
};

use StructureWithData::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectorData {
    pub inputs: &'static [Offset],
    pub outputs: &'static [Offset],
}

// TODO: individual content slots
// TODO: collision detection
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureWithData {
    AirPump,
    Refinery(Box<[Item; 12]>),
    Disharmonizer,
    Unifier,
    SubdimensionalMarket,
    Splitter,
    Merger,
    StorageVault(Box<[Item; 16]>),
    AbysalDoor,
    SingleStorage,
    Laboratory,
    RitualInfuser,
    BigMerger,
    BigSplitter,
}

impl StructureWithData {
    pub fn kind(&self) -> StructureKind {
        self.into()
    }

    pub fn get_storage(&self) -> &[Item] {
        match self {
            Refinery(x) => &**x,
            StorageVault(x) => &**x,
            _ => &[],
        }
    }

    fn export_stored_item(
        f: &mut impl Write,
        id: usize,
        item_index: usize,
        item: Item,
    ) -> fmt::Result {
        let item_id = item as i8;
        writeln!(f, "{id}-storage_load_at {item_index}=\"{item_id}.000000\"")
    }

    pub fn export(&self, f: &mut impl Write, id: usize, raw_x: i32, raw_y: i32) -> fmt::Result {
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

    fn export_struct(&self, f: &mut impl Write, id: usize) -> fmt::Result {
        match self {
            AirPump => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:1.0,+row+:1.0,+content_column+:1.0,+type+:1,+content_row+:0.0,+content+:-1.0}}],+type+:0.0,+machine_type+:{{+name+:+Air Pump+,+type+:0,+description+:+Sucks in potent air from the surrounding valley and puts it in a bottle.+,+sprite+:5,+machine_cost+:{{+cost_type_list+:[8,0,0,1,1,2,2,5,15,16,16,16,7,7,7,7,7,7,20,20,20,21,21,21,21,21,21],+cost_amount_list+:[3.0,2.0,4.0,4.0,4.0,4.0,3.0,4.0,5.0,3.0,3.0,3.0,4.0,4.0,3.0,3.0,2.0,2.0,3.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:8.0}},+input_list+:[]}}""#
            ),
            Refinery(_) => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:5.0,+row+:0.0,+content_column+:5.0,+type+:1,+content_row+:1.0,+content+:-1.0}}],+type+:1.0,+machine_type+:{{+name+:+Refinery+,+type+:1,+description+:+Improves a resource, turning it into something better.+,+sprite+:35,+machine_cost+:{{+cost_type_list+:[0,1,1,1,1,3,3,3,3,3,3,3,15,15,15,15,16,16,16],+cost_amount_list+:[3.0,2.0,2.0,2.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:0.0,+type+:0,+content_row+:1.0,+content+:-1.0}}]}}""#
            ),
            Disharmonizer => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:3.0,+row+:0.0,+content_column+:2.0,+type+:1,+content_row+:0.0,+content+:-1.0}},{{+index+:1.0,+column+:3.0,+row+:1.0,+content_column+:2.0,+type+:1,+content_row+:1.0,+content+:-1.0}},{{+index+:2.0,+column+:3.0,+row+:2.0,+content_column+:2.0,+type+:1,+content_row+:2.0,+content+:-1.0}},{{+index+:3.0,+column+:3.0,+row+:3.0,+content_column+:2.0,+type+:1,+content_row+:3.0,+content+:-1.0}}],+type+:2.0,+machine_type+:{{+name+:+Disharmonizer+,+type+:2,+description+:+Breaks resources apart by nature and magical sequence.+,+sprite+:37,+machine_cost+:{{+cost_type_list+:[1,5,5,15,17,17,17,17,17,17,17,17,18,18,18,18,18,18,20,20,20,20,20,21,21,21,21],+cost_amount_list+:[3.0,4.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,3.0,2.0,2.0,4.0,3.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:3.0,+content_column+:0.0,+type+:0,+content_row+:2.0,+content+:-1.0}}]}}""#
            ),
            Unifier => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:1.0,+row+:0.0,+content_column+:1.0,+type+:1,+content_row+:1.0,+content+:-1.0}}],+type+:3.0,+machine_type+:{{+name+:+Unifier+,+type+:3,+description+:+Converges multiple resources into one.+,+sprite+:61,+machine_cost+:{{+cost_type_list+:[2,15,15,15,15,15,7,7,7,16,16,16,16],+cost_amount_list+:[4.0,4.0,3.0,2.0,2.0,2.0,3.0,3.0,2.0,4.0,3.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:4.0,+content_column+:0.0,+type+:0,+content_row+:3.0,+content+:-1.0}},{{+index+:1.0,+column+:1.0,+row+:4.0,+content_column+:1.0,+type+:0,+content_row+:3.0,+content+:-1.0}},{{+index+:2.0,+column+:2.0,+row+:4.0,+content_column+:2.0,+type+:0,+content_row+:3.0,+content+:-1.0}}]}}""#
            ),
            SubdimensionalMarket => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:3.0,+row+:0.0,+content_column+:2.0,+type+:1.0,+content_row+:0.0,+content+:-1.0}},{{+index+:1.0,+column+:3.0,+row+:1.0,+content_column+:2.0,+type+:1.0,+content_row+:1.0,+content+:7}},{{+index+:2.0,+column+:3.0,+row+:2.0,+content_column+:2.0,+type+:1.0,+content_row+:2.0,+content+:6}}],+type+:4.0,+machine_type+:{{+name+:+Subdimensional Market+,+type+:4.0,+description+:+Sell any resource for coin. Some are more worth than others.+,+sprite+:52.0,+machine_cost+:{{+cost_type_list+:[10.0,11.0,11.0,8.0,8.0,21.0],+cost_amount_list+:[4.0,4.0,3.0,4.0,3.0,2.0]}},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:1.0,+machine_speed+:8.0}},+input_list+:[{{+index+:0.0,+column+:3.0,+row+:4.0,+content_column+:2.0,+type+:0.0,+content_row+:4.0,+content+:-1.0}}]}}""#
            ),
            Splitter => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:1.0,+content_row+:-1.0,+content+:-1.0}},{{+index+:1.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:1.0,+content_row+:-1.0,+content+:-1.0}}],+type+:5.0,+machine_type+:{{+name+:+Splitter+,+type+:5.0,+description+:+Split an incomming connection into two outputs.+,+sprite+:24.0,+machine_cost+:{{+cost_type_list+:[1.0,1.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,3.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:0.0,+content_row+:-1.0,+content+:-1.0}}]}}""#
            ),
            Merger => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:1.0,+content_row+:-1.0,+content+:-1.0}}],+type+:6.0,+machine_type+:{{+name+:+Merger+,+type+:6.0,+description+:+Merges two incomming connections into one output.+,+sprite+:25.0,+machine_cost+:{{+cost_type_list+:[1.0,1.0,1.0,1.0,4.0,4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,2.0,2.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:0.0,+content_row+:-1.0,+content+:-1.0}},{{+index+:1.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:0.0,+content_row+:-1.0,+content+:-1.0}}]}}""#
            ),
            StorageVault(_) => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:4.0,+row+:1.0,+content_column+:4.0,+type+:1.0,+content_row+:0.0,+content+:-1.0}}],+type+:7.0,+machine_type+:{{+name+:+Storage Vault+,+type+:7.0,+description+:+A machine which keeps your resources safe behind thick glass.+,+sprite+:6.0,+machine_cost+:{{+cost_type_list+:[4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:0.0,+type+:0.0,+content_row+:0.0,+content+:-1.0}}]}}""#
            ),
            AbysalDoor => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[],+type+:8.0,+machine_type+:{{+name+:+Abysal Door+,+type+:8.0,+description+:+Get rid of all you don't have a need for.+,+sprite+:3.0,+machine_cost+:{{+cost_type_list+:[2.0,2.0,2.0,2.0,2.0],+cost_amount_list+:[4.0,3.0,3.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:1.0,+machine_speed+:2.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:1.0,+type+:0.0,+content_row+:0.0,+content+:-1.0}}]}}""#
            ),
            SingleStorage => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[],+type+:9.0,+machine_type+:{{+name+:+Single Storage+,+type+:9.0,+description+:+A single storage place for a single resource.+,+sprite+:17.0,+machine_cost+:{{+cost_type_list+:[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,5.0,5.0,5.0],+cost_amount_list+:[2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0}},+input_list+:[{{+index+:0.0,+column+:-1.0,+row+:-1.0,+content_column+:0.0,+type+:0.0,+content_row+:0.0,+content+:-1.0}}]}}""#
            ),
            Laboratory => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[],+type+:10.0,+machine_type+:{{+name+:+Laboratory+,+type+:10.0,+description+:+Used to research more stuff.+,+sprite+:45.0,+machine_cost+:{{+cost_type_list+:[8.0],+cost_amount_list+:[100.0]}},+cost_input+:10.0,+speed_increase+:32.0,+unlocked+:0.0,+machine_speed+:4.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:0.0,+type+:0.0,+content_row+:0.0,+content+:-1.0}}]}}""#
            ),
            RitualInfuser => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:-1.0,+row+:-1.0,+content_column+:2.0,+type+:1.0,+content_row+:3.0,+content+:-1.0}}],+type+:11.0,+machine_type+:{{+name+:+Ritual Infuser+,+type+:11.0,+description+:+Automate magical rituals. Used to create the phylactery.+,+sprite+:44.0,+machine_cost+:{{+cost_type_list+:[21.0,25.0,25.0,25.0,25.0],+cost_amount_list+:[8.0,2.0,2.0,1.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:1.0,+content_column+:1.0,+type+:0.0,+content_row+:1.0,+content+:-1.0}},{{+index+:1.0,+column+:2.0,+row+:0.0,+content_column+:2.0,+type+:0.0,+content_row+:1.0,+content+:-1.0}},{{+index+:2.0,+column+:4.0,+row+:1.0,+content_column+:3.0,+type+:0.0,+content_row+:1.0,+content+:-1.0}}]}}""#
            ),
            BigMerger => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:5.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:-1.0}}],+type+:12,+machine_type+:{{+name+:+Big Merger+,+type+:12,+description+:+Merges Inputs. Lowest always first.+,+sprite+:53,+machine_cost+:{{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:-1.0}},{{+index+:1.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:-1.0}},{{+index+:2.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:-1.0}},{{+index+:3.0,+column+:0.0,+row+:3.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:-1.0}},{{+index+:4.0,+column+:0.0,+row+:4.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:-1.0}}]}}""#
            ),
            BigSplitter => writeln!(
                f,
                r#"{id}-struct="{{+output_list+:[{{+index+:0.0,+column+:0.0,+row+:0.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:-1.0}},{{+index+:1.0,+column+:0.0,+row+:1.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:-1.0}},{{+index+:2.0,+column+:0.0,+row+:2.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:-1.0}},{{+index+:3.0,+column+:0.0,+row+:3.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:-1.0}},{{+index+:4.0,+column+:0.0,+row+:4.0,+content_column+:-1.0,+type+:1,+content_row+:-1.0,+content+:-1.0}}],+type+:13,+machine_type+:{{+name+:+Big Splitter+,+type+:13,+description+:+Splits Outputs. Lowest always first.+,+sprite+:22,+machine_cost+:{{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]}},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:1.0}},+input_list+:[{{+index+:0.0,+column+:0.0,+row+:5.0,+content_column+:-1.0,+type+:0,+content_row+:-1.0,+content+:-1.0}}]}}""#
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
    AbysalDoor,
    SingleStorage,
    Laboratory,
    RitualInfuser,
    BigMerger,
    BigSplitter,
}

impl StructureKind {
    pub fn size(&self) -> (u8, u8) {
        match self {
            Self::AirPump => (2, 2),
            Self::Refinery => (6, 2),
            Self::Disharmonizer => (4, 4),
            Self::Unifier => (4, 5),
            Self::SubdimensionalMarket => (4, 5),
            Self::Splitter => (1, 3),
            Self::Merger => (1, 3),
            Self::StorageVault => (5, 2),
            Self::AbysalDoor => (4, 1),
            Self::SingleStorage => (1, 1),
            Self::Laboratory => (5, 2),
            Self::RitualInfuser => (5, 5),
            Self::BigMerger => (1, 6),
            Self::BigSplitter => (1, 6),
        }
    }

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

    pub fn storage_capacity(&self) -> usize {
        match self {
            Self::Refinery => 12,
            Self::StorageVault => 16,
            _ => 0,
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
                outputs: &[ /*-1, -1*/ ],
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
                outputs: &[ /*-1, -1*/ ],
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

impl From<&StructureWithData> for StructureKind {
    fn from(value: &StructureWithData) -> Self {
        match value {
            AirPump => Self::AirPump,
            Refinery(_) => Self::Refinery,
            Disharmonizer => Self::Disharmonizer,
            Unifier => Self::Unifier,
            SubdimensionalMarket => Self::SubdimensionalMarket,
            Splitter => Self::Splitter,
            Merger => Self::Merger,
            StorageVault(_) => Self::StorageVault,
            AbysalDoor => Self::AbysalDoor,
            SingleStorage => Self::SingleStorage,
            Laboratory => Self::Laboratory,
            RitualInfuser => Self::RitualInfuser,
            BigMerger => Self::BigMerger,
            BigSplitter => Self::BigSplitter,
        }
    }
}

impl From<StructureKind> for StructureWithData {
    fn from(value: StructureKind) -> Self {
        match value {
            StructureKind::AirPump => AirPump,
            StructureKind::Refinery => Refinery(Box::default()),
            StructureKind::Disharmonizer => Disharmonizer,
            StructureKind::Unifier => Unifier,
            StructureKind::SubdimensionalMarket => SubdimensionalMarket,
            StructureKind::Splitter => Splitter,
            StructureKind::Merger => Merger,
            StructureKind::StorageVault => StorageVault(Box::default()),
            StructureKind::AbysalDoor => AbysalDoor,
            StructureKind::SingleStorage => SingleStorage,
            StructureKind::Laboratory => Laboratory,
            StructureKind::RitualInfuser => RitualInfuser,
            StructureKind::BigMerger => BigMerger,
            StructureKind::BigSplitter => BigSplitter,
        }
    }
}
