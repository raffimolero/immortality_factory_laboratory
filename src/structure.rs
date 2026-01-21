use crate::prelude::*;

use std::fmt::{self, Display};
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectorData {
    pub port: Offset,
    pub slot: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IoData {
    pub inputs: &'static [ConnectorData],
    pub outputs: &'static [ConnectorData],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PortOutData {
    pub item: Item,
    target: Option<PortIn>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PortInData {
    pub item: Item,
    target: Option<PortOut>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureData {
    AirPump {
        outputs: [PortOutData; 1],
    },
    Refinery {
        inputs: [PortInData; 1],
        storage: [Item; 12],
        outputs: [PortOutData; 1],
    },
    Disharmonizer {
        inputs: [PortInData; 1],
        outputs: [PortOutData; 4],
    },
    Unifier {
        inputs: [PortInData; 3],
        outputs: [PortOutData; 1],
    },
    SubdimensionalMarket {
        inputs: [PortInData; 1],
        outputs: [PortOutData; 3],
    },
    Splitter {
        inputs: [PortInData; 1],
        outputs: [PortOutData; 2],
    },
    Merger {
        inputs: [PortInData; 2],
        outputs: [PortOutData; 1],
    },
    StorageVault {
        inputs: [PortInData; 1],
        storage: [Item; 16],
        outputs: [PortOutData; 1],
    },
    AbysalDoor {
        inputs: [PortInData; 1],
    },
    SingleStorage {
        inputs: [PortInData; 1],
    },
    Laboratory {
        inputs: [PortInData; 1],
    },
    RitualInfuser {
        inputs: [PortInData; 3],
        outputs: [PortOutData; 1],
    },
    BigMerger {
        inputs: [PortInData; 5],
        outputs: [PortOutData; 1],
    },
    BigSplitter {
        inputs: [PortInData; 1],
        outputs: [PortOutData; 5],
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

    pub fn get_inputs(&self) -> &[PortInData] {
        use StructureData::*;
        match self {
            AirPump { .. } => &[],
            Refinery { inputs, .. } => inputs,
            Disharmonizer { inputs, .. } => inputs,
            Unifier { inputs, .. } => inputs,
            SubdimensionalMarket { inputs, .. } => inputs,
            Splitter { inputs, .. } => inputs,
            Merger { inputs, .. } => inputs,
            StorageVault { inputs, .. } => inputs,
            AbysalDoor { inputs, .. } => inputs,
            SingleStorage { inputs } => inputs,
            Laboratory { inputs, .. } => inputs,
            RitualInfuser { inputs, .. } => inputs,
            BigMerger { inputs, .. } => inputs,
            BigSplitter { inputs, .. } => inputs,
        }
    }
    pub fn get_outputs(&self) -> &[PortOutData] {
        use StructureData::*;
        match self {
            AbysalDoor { .. } | SingleStorage { .. } | Laboratory { .. } | RitualInfuser { .. } => {
                &[]
            }
            AirPump { outputs, .. } => outputs,
            Refinery { outputs, .. } => outputs,
            Disharmonizer { outputs, .. } => outputs,
            Unifier { outputs, .. } => outputs,
            SubdimensionalMarket { outputs, .. } => outputs,
            Splitter { outputs, .. } => outputs,
            Merger { outputs, .. } => outputs,
            StorageVault { outputs, .. } => outputs,
            BigMerger { outputs, .. } => outputs,
            BigSplitter { outputs, .. } => outputs,
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

    pub fn export(
        &self,
        f: &mut impl Write,
        id: usize,
        raw_x: Coord,
        raw_y: Coord,
    ) -> io::Result<()> {
        let world_y = raw_y as i32 * 22;
        let world_x = raw_x as i32 * 22;
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
    //
    // fn export_with(&self, f: &mut impl Write, id: usize, output_list: &[PortOutData], machine_type: &str, input_list: &[PortInData]) -> io::Result<()> {
    //     writeln!(f, r#"{id}-struct="{{+output_list+:["#)?;
    //     writeln!(f, r#"]+type+:0.0,+machine_type+:}}""}#)?;
    //
    // }

    fn export_struct(&self, f: &mut impl Write, id: usize, world: &World) -> io::Result<()> {
        let machine_type = match *self {
            Self::AirPump { .. } => {
                "+type+:0.0,+machine_type+:{+name+:+Air Pump+,+type+:0,+description+:+Sucks in potent air from the surrounding valley and puts it in a bottle.+,+sprite+:5,+machine_cost+:{+cost_type_list+:[8,0,0,1,1,2,2,5,15,16,16,16,7,7,7,7,7,7,20,20,20,21,21,21,21,21,21],+cost_amount_list+:[3.0,2.0,4.0,4.0,4.0,4.0,3.0,4.0,5.0,3.0,3.0,3.0,4.0,4.0,3.0,3.0,2.0,2.0,3.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:8.0},"
            }
            Self::Refinery { .. } => {
                "+type+:1.0,+machine_type+:{+name+:+Refinery+,+type+:1,+description+:+Improves a resource, turning it into something better.+,+sprite+:35,+machine_cost+:{+cost_type_list+:[0,1,1,1,1,3,3,3,3,3,3,3,15,15,15,15,16,16,16],+cost_amount_list+:[3.0,2.0,2.0,2.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}"
            }
            Self::Disharmonizer { .. } => {
                "+type+:2.0,+machine_type+:{+name+:+Disharmonizer+,+type+:2,+description+:+Breaks resources apart by nature and magical sequence.+,+sprite+:37,+machine_cost+:{+cost_type_list+:[1,5,5,15,17,17,17,17,17,17,17,17,18,18,18,18,18,18,20,20,20,20,20,21,21,21,21],+cost_amount_list+:[3.0,4.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,3.0,2.0,2.0,4.0,3.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0},"
            }
            Self::Unifier { .. } => {
                "+type+:3.0,+machine_type+:{+name+:+Unifier+,+type+:3,+description+:+Converges multiple resources into one.+,+sprite+:61,+machine_cost+:{+cost_type_list+:[2,15,15,15,15,15,7,7,7,16,16,16,16],+cost_amount_list+:[4.0,4.0,3.0,2.0,2.0,2.0,3.0,3.0,2.0,4.0,3.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0},"
            }
            Self::SubdimensionalMarket { .. } => {
                "+type+:4.0,+machine_type+:{+name+:+Subdimensional Market+,+type+:4.0,+description+:+Sell any resource for coin. Some are more worth than others.+,+sprite+:52.0,+machine_cost+:{+cost_type_list+:[10.0,11.0,11.0,8.0,8.0,21.0],+cost_amount_list+:[4.0,4.0,3.0,4.0,3.0,2.0]},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:1.0,+machine_speed+:8.0},"
            }
            Self::Splitter { .. } => {
                "+type+:5.0,+machine_type+:{+name+:+Splitter+,+type+:5.0,+description+:+Split an incomming connection into two outputs.+,+sprite+:24.0,+machine_cost+:{+cost_type_list+:[1.0,1.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,3.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0},"
            }
            Self::Merger { .. } => {
                "+type+:6.0,+machine_type+:{+name+:+Merger+,+type+:6.0,+description+:+Merges two incomming connections into one output.+,+sprite+:25.0,+machine_cost+:{+cost_type_list+:[1.0,1.0,1.0,1.0,4.0,4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,2.0,2.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0},"
            }
            Self::StorageVault { .. } => {
                "+type+:7.0,+machine_type+:{+name+:+Storage Vault+,+type+:7.0,+description+:+A machine which keeps your resources safe behind thick glass.+,+sprite+:6.0,+machine_cost+:{+cost_type_list+:[4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0},"
            }
            Self::AbysalDoor { .. } => {
                "+type+:8.0,+machine_type+:{+name+:+Abysal Door+,+type+:8.0,+description+:+Get rid of all you don't have a need for.+,+sprite+:3.0,+machine_cost+:{+cost_type_list+:[2.0,2.0,2.0,2.0,2.0],+cost_amount_list+:[4.0,3.0,3.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:1.0,+machine_speed+:2.0},"
            }
            Self::SingleStorage { .. } => {
                "+type+:9.0,+machine_type+:{+name+:+Single Storage+,+type+:9.0,+description+:+A single storage place for a single resource.+,+sprite+:17.0,+machine_cost+:{+cost_type_list+:[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,5.0,5.0,5.0],+cost_amount_list+:[2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:-1.0},"
            }
            Self::Laboratory { .. } => {
                "+type+:10.0,+machine_type+:{+name+:+Laboratory+,+type+:10.0,+description+:+Used to research more stuff.+,+sprite+:45.0,+machine_cost+:{+cost_type_list+:[8.0],+cost_amount_list+:[100.0]},+cost_input+:10.0,+speed_increase+:32.0,+unlocked+:0.0,+machine_speed+:4.0},"
            }
            Self::RitualInfuser { .. } => {
                "+type+:11.0,+machine_type+:{+name+:+Ritual Infuser+,+type+:11.0,+description+:+Automate magical rituals. Used to create the phylactery.+,+sprite+:44.0,+machine_cost+:{+cost_type_list+:[21.0,25.0,25.0,25.0,25.0],+cost_amount_list+:[8.0,2.0,2.0,1.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:1.0,+machine_speed+:1.0},"
            }
            Self::BigMerger { .. } => {
                "+type+:12,+machine_type+:{+name+:+Big Merger+,+type+:12,+description+:+Merges Inputs. Lowest always first.+,+sprite+:53,+machine_cost+:{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-10},"
            }
            Self::BigSplitter { .. } => {
                "+type+:13,+machine_type+:{+name+:+Big Splitter+,+type+:13,+description+:+Splits Outputs. Lowest always first.+,+sprite+:22,+machine_cost+:{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-10},"
            }
        };
        write!(f, r#"{id}-struct="{{+output_list+:["#)?;
        for (
            i,
            (
                ConnectorData {
                    port: Offset { x: px, y: py },
                    slot: Offset { x: sx, y: sy },
                },
                PortOutData { item, target },
            ),
        ) in self
            .kind()
            .connectors()
            .outputs
            .iter()
            .zip(self.get_outputs())
            .enumerate()
        {
            let item_id = *item as i8;
            let (target_port, target_id, target_x, target_y) =
                target.map_or((-1, -1, -1, -1), |port| {
                    let i = port.structure_id.index;
                    let (tx, ty) = world.structures[i].pos.world_coords();
                    (port.index as i8, i as i32 + 100000, tx, ty)
                });
            write!(
                f,
                r#"{{+index+:{i}.0,+column+:{px}.0,+row+:{py}.0,+content_column+:{sx}.0,+type+:1,+content_row+:{sy}.0,+content+:{item_id}.0,+connected_machine+:{target_id},+connected_machine_slot_index+:{target_port}.0,+connected_machine_x+:{target_x}.0,+connected_machine_y+:{target_y}.0}}"#
            )?;
        }
        write!(f, "{machine_type}")?;
        for (
            i,
            (
                ConnectorData {
                    port: Offset { x: px, y: py },
                    slot: Offset { x: sx, y: sy },
                },
                PortInData { item, target },
            ),
        ) in self
            .kind()
            .connectors()
            .inputs
            .iter()
            .zip(self.get_inputs())
            .enumerate()
        {
            let item_id = *item as i8;
            let (target_port, target_id, target_x, target_y) =
                target.map_or((-1, -1, -1, -1), |port| {
                    let i = port.structure_id.index;
                    let (tx, ty) = world.structures[i].pos.world_coords();
                    (port.index as i8, i as i32 + 100000, tx, ty)
                });
            write!(
                f,
                r#"{{+index+:{i}.0,+column+:{px}.0,+row+:{py}.0,+content_column+:{sx}.0,+type+:0,+content_row+:{sy}.0,+content+:{item_id}.0,+connected_machine+:{target_id},+connected_machine_slot_index+:{target_port}.0,+connected_machine_x+:{target_x}.0,+connected_machine_y+:{target_y}.0}}"#
            )?;
        }
        writeln!(f, r#"]}}""#)
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

    pub fn connectors(&self) -> IoData {
        match self {
            Self::AirPump => IoData {
                inputs: &[],
                outputs: &[Offset { x: 1, y: 1 }],
            },
            Self::Refinery => IoData {
                inputs: &[Offset { x: 0, y: 0 }],
                outputs: &[Offset { x: 5, y: 0 }],
            },
            Self::Disharmonizer => IoData {
                inputs: &[Offset { x: 0, y: 3 }],
                outputs: &[
                    Offset { x: 3, y: 0 },
                    Offset { x: 3, y: 1 },
                    Offset { x: 3, y: 2 },
                    Offset { x: 3, y: 3 },
                ],
            },
            Self::Unifier => IoData {
                inputs: &[
                    Offset { x: 0, y: 4 },
                    Offset { x: 1, y: 4 },
                    Offset { x: 2, y: 4 },
                ],
                outputs: &[Offset { x: 1, y: 0 }],
            },
            Self::SubdimensionalMarket => IoData {
                inputs: &[Offset { x: 3, y: 4 }],
                outputs: &[
                    Offset { x: 3, y: 0 },
                    Offset { x: 3, y: 1 },
                    Offset { x: 3, y: 2 },
                ],
            },
            Self::Splitter => IoData {
                inputs: &[Offset { x: 0, y: 1 }],
                outputs: &[Offset { x: 0, y: 0 }, Offset { x: 0, y: 2 }],
            },
            Self::Merger => IoData {
                inputs: &[Offset { x: 0, y: 0 }, Offset { x: 0, y: 2 }],
                outputs: &[Offset { x: 0, y: 1 }],
            },
            Self::StorageVault => IoData {
                inputs: &[Offset { x: 0, y: 1 }],
                outputs: &[Offset { x: 4, y: 1 }],
            },
            Self::AbysalDoor => IoData {
                inputs: &[Offset { x: 0, y: 0 }],
                outputs: &[],
            },
            Self::SingleStorage => IoData {
                inputs: &[],
                outputs: &[Offset::NULL],
            },
            Self::Laboratory => IoData {
                inputs: &[Offset { x: 0, y: 1 }],
                outputs: &[],
            },
            Self::RitualInfuser => IoData {
                inputs: &[
                    Offset { x: 0, y: 1 },
                    Offset { x: 2, y: 0 },
                    Offset { x: 4, y: 1 },
                ],
                outputs: &[Offset::NULL],
            },
            Self::BigMerger => IoData {
                inputs: &[
                    Offset { x: 0, y: 0 },
                    Offset { x: 0, y: 1 },
                    Offset { x: 0, y: 2 },
                    Offset { x: 0, y: 3 },
                    Offset { x: 0, y: 4 },
                ],
                outputs: &[Offset { x: 0, y: 5 }],
            },
            Self::BigSplitter => IoData {
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
        match value {
            StructureKind::AirPump => Self::AirPump {
                outputs: Default::default(),
            },
            StructureKind::Refinery => Self::Refinery {
                inputs: Default::default(),
                storage: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::Disharmonizer => Self::Disharmonizer {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::Unifier => Self::Unifier {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::SubdimensionalMarket => Self::SubdimensionalMarket {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::Splitter => Self::Splitter {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::Merger => Self::Merger {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::StorageVault => Self::StorageVault {
                inputs: Default::default(),
                storage: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::AbysalDoor => Self::AbysalDoor {
                inputs: Default::default(),
            },
            StructureKind::SingleStorage => Self::SingleStorage {
                inputs: Default::default(),
            },
            StructureKind::Laboratory => Self::Laboratory {
                inputs: Default::default(),
            },
            StructureKind::RitualInfuser => Self::RitualInfuser {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::BigMerger => Self::BigMerger {
                inputs: Default::default(),
                outputs: Default::default(),
            },
            StructureKind::BigSplitter => Self::BigSplitter {
                inputs: Default::default(),
                outputs: Default::default(),
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
