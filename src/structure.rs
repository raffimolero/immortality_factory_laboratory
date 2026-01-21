use crate::prelude::*;
use crate::world::Position;

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
pub(crate) struct PortOutRaw {
    pub(crate) structure_index: usize,
    pub(crate) port: u8,
}

impl From<PortOut> for PortOutRaw {
    fn from(value: PortOut) -> Self {
        Self {
            structure_index: value.structure_id.index,
            port: value.index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct PortInRaw {
    pub(crate) structure_index: usize,
    pub(crate) port: u8,
}

impl From<PortIn> for PortInRaw {
    fn from(value: PortIn) -> Self {
        Self {
            structure_index: value.structure_id.index,
            port: value.index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PortOutData {
    pub item: Item,
    pub(crate) target: Option<PortInRaw>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct PortInData {
    pub item: Item,
    pub(crate) target: Option<PortOutRaw>,
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
        // technically considered an input in the code
        input: Item,
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

impl From<StructureData> for StructureDataFull {
    fn from(value: StructureData) -> Self {
        use StructureData::*;
        fn i(item: Item) -> PortInData {
            PortInData { item, target: None }
        }
        fn o(item: Item) -> PortOutData {
            PortOutData { item, target: None }
        }

        match value {
            AirPump { output } => StructureDataFull::AirPump {
                outputs: [o(output)],
            },
            Refinery {
                input,
                storage,
                output,
            } => StructureDataFull::Refinery {
                inputs: [i(input)],
                storage,
                outputs: [o(output)],
            },
            Disharmonizer { input, outputs } => StructureDataFull::Disharmonizer {
                inputs: [i(input)],
                outputs: outputs.map(o),
            },
            Unifier { inputs, output } => StructureDataFull::Unifier {
                inputs: inputs.map(i),
                outputs: [o(output)],
            },
            SubdimensionalMarket { input, outputs } => StructureDataFull::SubdimensionalMarket {
                inputs: [i(input)],
                outputs: outputs.map(o),
            },
            Splitter { input, outputs } => StructureDataFull::Splitter {
                inputs: [i(input)],
                outputs: outputs.map(o),
            },
            Merger { inputs, output } => StructureDataFull::Merger {
                inputs: inputs.map(i),
                outputs: [o(output)],
            },
            StorageVault {
                input,
                storage,
                output,
            } => StructureDataFull::StorageVault {
                inputs: [i(input)],
                storage,
                outputs: [o(output)],
            },
            AbysalDoor { input } => StructureDataFull::AbysalDoor { inputs: [i(input)] },
            SingleStorage {
                // technically considered an input in the code
                input,
            } => StructureDataFull::SingleStorage { inputs: [i(input)] },
            Laboratory { input } => StructureDataFull::Laboratory { inputs: [i(input)] },
            RitualInfuser { inputs, output } => StructureDataFull::RitualInfuser {
                inputs: inputs.map(i),
                outputs: [o(output)],
            },
            BigMerger { inputs, output } => StructureDataFull::BigMerger {
                inputs: inputs.map(i),
                outputs: [o(output)],
            },
            BigSplitter { input, outputs } => StructureDataFull::BigSplitter {
                inputs: [i(input)],
                outputs: outputs.map(o),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StructureDataFull {
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

impl StructureDataFull {
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
        use StructureDataFull::*;
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

    pub fn get_inputs_mut(&mut self) -> &mut [PortInData] {
        use StructureDataFull::*;
        match self {
            AirPump { .. } => &mut [],
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
        use StructureDataFull::*;
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

    pub fn get_outputs_mut(&mut self) -> &mut [PortOutData] {
        use StructureDataFull::*;
        match self {
            AbysalDoor { .. } | SingleStorage { .. } | Laboratory { .. } | RitualInfuser { .. } => {
                &mut []
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
        world: &World,
        id: usize,
        raw_x: Coord,
        raw_y: Coord,
    ) -> io::Result<()> {
        let (world_x, world_y) = Position { x: raw_x, y: raw_y }.world_coords();
        let obj_num = self.kind().object_number();

        write!(f, r#"{id}-struct="{{+output_list+:["#)?;
        let mut comma = false;
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
                target.map_or((-1, -1, 0, 0), |port| {
                    let i = port.structure_index;
                    let (tx, ty) = world.structures[i].pos.world_coords();
                    (port.port as i8, i as i32 + 100000, tx, ty)
                });
            if comma {
                write!(f, ",")?;
            }
            comma = true;
            write!(
                f,
                r#"{{+index+:{i}.0,+column+:{px}.0,+row+:{py}.0,+content_column+:{sx}.0,+type+:1,+content_row+:{sy}.0,+content+:{item_id}.0,+connected_machine+:{target_id},+connected_machine_slot_index+:{target_port}.0,+connected_machine_x+:{target_x}.0,+connected_machine_y+:{target_y}.0}}"#
            )?;
        }
        write!(f, "],{},+input_list+:[", self.machine_type())?;
        let mut comma = false;
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
                target.map_or((-1, -1, 0, 0), |port| {
                    let i = port.structure_index;
                    let (tx, ty) = world.structures[i].pos.world_coords();
                    (port.port as i8, i as i32 + 100000, tx, ty)
                });
            if comma {
                write!(f, ",")?;
            }
            comma = true;
            write!(
                f,
                r#"{{+index+:{i}.0,+column+:{px}.0,+row+:{py}.0,+content_column+:{sx}.0,+type+:0,+content_row+:{sy}.0,+content+:{item_id}.0,+connected_machine+:{target_id},+connected_machine_slot_index+:{target_port}.0,+connected_machine_x+:{target_x}.0,+connected_machine_y+:{target_y}.0}}"#
            )?;
        }
        writeln!(f, r#"]}}""#)?;
        writeln!(f, "{id}-y=\"{world_y}.000000\"")?;
        writeln!(f, "{id}-x=\"{world_x}.000000\"")?;
        writeln!(f, "{id}-object=\"{obj_num}.000000\"")?;
        for (i, item) in self.get_storage().iter().enumerate() {
            Self::export_stored_item(f, id, i, *item)?;
        }
        Ok(())
    }

    fn machine_type(&self) -> &'static str {
        match *self {
            Self::AirPump { .. } => {
                "+type+:0,+machine_type+:{+name+:+Air Pump+,+type+:0,+description+:+Sucks in potent air from the surrounding valley and puts it in a bottle.+,+sprite+:5,+machine_cost+:{+cost_type_list+:[8,0,0,1,1,2,2,5,15,16,16,16,7,7,7,7,7,7,20,20,20,21,21,21,21,21,21],+cost_amount_list+:[3.0,2.0,4.0,4.0,4.0,4.0,3.0,4.0,5.0,3.0,3.0,3.0,4.0,4.0,3.0,3.0,2.0,2.0,3.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:8.0}"
            }
            Self::Refinery { .. } => {
                "+type+:1,+machine_type+:{+name+:+Refinery+,+type+:1,+description+:+Improves a resource, turning it into something better.+,+sprite+:35,+machine_cost+:{+cost_type_list+:[0,1,1,1,1,3,3,3,3,3,3,3,15,15,15,15,16,16,16],+cost_amount_list+:[3.0,2.0,2.0,2.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.}"
            }
            Self::Disharmonizer { .. } => {
                "+type+:2,+machine_type+:{+name+:+Disharmonizer+,+type+:2,+description+:+Breaks resources apart by nature and magical sequence.+,+sprite+:37,+machine_cost+:{+cost_type_list+:[1,5,5,15,17,17,17,17,17,17,17,17,18,18,18,18,18,18,20,20,20,20,20,21,21,21,21],+cost_amount_list+:[3.0,4.0,2.0,4.0,3.0,3.0,2.0,2.0,2.0,2.0,2.0,2.0,3.0,3.0,2.0,2.0,2.0,2.0,3.0,3.0,3.0,2.0,2.0,4.0,3.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}"
            }
            Self::Unifier { .. } => {
                "+type+:3,+machine_type+:{+name+:+Unifier+,+type+:3,+description+:+Converges multiple resources into one.+,+sprite+:61,+machine_cost+:{+cost_type_list+:[2,15,15,15,15,15,7,7,7,16,16,16,16],+cost_amount_list+:[4.0,4.0,3.0,2.0,2.0,2.0,3.0,3.0,2.0,4.0,3.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:8.0,+unlocked+:true,+machine_speed+:16.0}"
            }
            Self::SubdimensionalMarket { .. } => {
                "+type+:4,+machine_type+:{+name+:+Subdimensional Market+,+type+:4.0,+description+:+Sell any resource for coin. Some are more worth than others.+,+sprite+:52,+machine_cost+:{+cost_type_list+:[10.0,11.0,11.0,8.0,8.0,21.0],+cost_amount_list+:[4.0,4.0,3.0,4.0,3.0,2.0]},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:true,+machine_speed+:8.0}"
            }
            Self::Splitter { .. } => {
                "+type+:5,+machine_type+:{+name+:+Splitter+,+type+:5.0,+description+:+Split an incomming connection into two outputs.+,+sprite+:24,+machine_cost+:{+cost_type_list+:[1.0,1.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,3.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-1.0}"
            }
            Self::Merger { .. } => {
                "+type+:6,+machine_type+:{+name+:+Merger+,+type+:6.0,+description+:+Merges two incomming connections into one output.+,+sprite+:25,+machine_cost+:{+cost_type_list+:[1.0,1.0,1.0,1.0,4.0,4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,2.0,2.0,3.0,2.0,3.0,3.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-1.0}"
            }
            Self::StorageVault { .. } => {
                "+type+:7,+machine_type+:{+name+:+Storage Vault+,+type+:7.0,+description+:+A machine which keeps your resources safe behind thick glass.+,+sprite+:6,+machine_cost+:{+cost_type_list+:[4.0,5.0,5.0,5.0,5.0,5.0],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-1.0}"
            }
            Self::AbysalDoor { .. } => {
                "+type+:8,+machine_type+:{+name+:+Abysal Door+,+type+:8.0,+description+:+Get rid of all you don't have a need for.+,+sprite+:3,+machine_cost+:{+cost_type_list+:[2.0,2.0,2.0,2.0,2.0],+cost_amount_list+:[4.0,3.0,3.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:4.0,+unlocked+:true,+machine_speed+:2.0}"
            }
            Self::SingleStorage { .. } => {
                "+type+:9,+machine_type+:{+name+:+Single Storage+,+type+:9.0,+description+:+A single storage place for a single resource.+,+sprite+:17,+machine_cost+:{+cost_type_list+:[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,1.0,5.0,5.0,5.0],+cost_amount_list+:[2.0,2.0,2.0,2.0,2.0,2.0,2.0,2.0,1.0,1.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-1.0}"
            }
            Self::Laboratory { .. } => {
                "+type+:10,+machine_type+:{+name+:+Laboratory+,+type+:10.0,+description+:+Used to research more stuff.+,+sprite+:45,+machine_cost+:{+cost_type_list+:[8],+cost_amount_list+:[100.0]},+cost_input+:10.0,+speed_increase+:32.0,+unlocked+:false,+machine_speed+:4.0}"
            }
            Self::RitualInfuser { .. } => {
                "+type+:11,+machine_type+:{+name+:+Ritual Infuser+,+type+:11.0,+description+:+Automate magical rituals. Used to create the phylactery.+,+sprite+:44,+machine_cost+:{+cost_type_list+:[21.0,25.0,25.0,25.0,25.0],+cost_amount_list+:[8.0,2.0,2.0,1.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:1.0}"
            }
            Self::BigMerger { .. } => {
                "+type+:12,+machine_type+:{+name+:+Big Merger+,+type+:12,+description+:+Merges Inputs. Lowest always first.+,+sprite+:53,+machine_cost+:{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-10}"
            }
            Self::BigSplitter { .. } => {
                "+type+:13,+machine_type+:{+name+:+Big Splitter+,+type+:13,+description+:+Splits Outputs. Lowest always first.+,+sprite+:22,+machine_cost+:{+cost_type_list+:[5,5,5,5,5,5,5],+cost_amount_list+:[3.0,3.0,3.0,2.0,2.0,2.0,1.0]},+cost_input+:0.0,+speed_increase+:1.0,+unlocked+:true,+machine_speed+:-10}"
            }
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

    pub fn connectors(&self) -> IoData {
        match self {
            Self::AirPump => IoData {
                inputs: &[],
                outputs: &[ConnectorData {
                    port: Offset { x: 1, y: 1 },
                    slot: Offset { x: 1, y: 0 },
                }],
            },
            Self::Refinery => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 0 },
                    slot: Offset { x: 0, y: 1 },
                }],
                outputs: &[ConnectorData {
                    port: Offset { x: 5, y: 0 },
                    slot: Offset { x: 5, y: 1 },
                }],
            },
            Self::Disharmonizer => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 3 },
                    slot: Offset { x: 0, y: 2 },
                }],
                outputs: &[
                    ConnectorData {
                        port: Offset { x: 3, y: 0 },
                        slot: Offset { x: 4, y: 0 },
                    },
                    ConnectorData {
                        port: Offset { x: 3, y: 1 },
                        slot: Offset { x: 4, y: 1 },
                    },
                    ConnectorData {
                        port: Offset { x: 3, y: 2 },
                        slot: Offset { x: 4, y: 2 },
                    },
                    ConnectorData {
                        port: Offset { x: 3, y: 3 },
                        slot: Offset { x: 4, y: 3 },
                    },
                ],
            },
            Self::Unifier => IoData {
                inputs: &[
                    ConnectorData {
                        port: Offset { x: 0, y: 4 },
                        slot: Offset { x: 0, y: 3 },
                    },
                    ConnectorData {
                        port: Offset { x: 1, y: 4 },
                        slot: Offset { x: 1, y: 3 },
                    },
                    ConnectorData {
                        port: Offset { x: 2, y: 4 },
                        slot: Offset { x: 2, y: 3 },
                    },
                ],
                outputs: &[ConnectorData {
                    port: Offset { x: 1, y: 0 },
                    slot: Offset { x: 1, y: 1 },
                }],
            },
            Self::SubdimensionalMarket => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 3, y: 4 },
                    slot: Offset { x: 2, y: 4 },
                }],
                outputs: &[
                    ConnectorData {
                        port: Offset { x: 3, y: 0 },
                        slot: Offset { x: 2, y: 0 },
                    },
                    ConnectorData {
                        port: Offset { x: 3, y: 1 },
                        slot: Offset { x: 2, y: 1 },
                    },
                    ConnectorData {
                        port: Offset { x: 3, y: 2 },
                        slot: Offset { x: 2, y: 2 },
                    },
                ],
            },
            Self::Splitter => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 1 },
                    slot: Offset::NULL,
                }],
                outputs: &[
                    ConnectorData {
                        port: Offset { x: 0, y: 0 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 2 },
                        slot: Offset::NULL,
                    },
                ],
            },
            Self::Merger => IoData {
                inputs: &[
                    ConnectorData {
                        port: Offset { x: 0, y: 0 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 2 },
                        slot: Offset::NULL,
                    },
                ],
                outputs: &[ConnectorData {
                    port: Offset { x: 0, y: 1 },
                    slot: Offset::NULL,
                }],
            },
            Self::StorageVault => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 1 },
                    slot: Offset { x: 0, y: 0 },
                }],
                outputs: &[ConnectorData {
                    port: Offset { x: 4, y: 1 },
                    slot: Offset { x: 4, y: 0 },
                }],
            },
            Self::AbysalDoor => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 0 },
                    slot: Offset { x: 1, y: 0 },
                }],
                outputs: &[],
            },
            Self::SingleStorage => IoData {
                inputs: &[],
                outputs: &[ConnectorData {
                    port: Offset::NULL,
                    slot: Offset { x: 0, y: 0 },
                }],
            },
            Self::Laboratory => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 1 },
                    slot: Offset { x: 0, y: 0 },
                }],
                outputs: &[],
            },
            Self::RitualInfuser => IoData {
                inputs: &[
                    ConnectorData {
                        port: Offset { x: 0, y: 1 },
                        slot: Offset { x: 1, y: 1 },
                    },
                    ConnectorData {
                        port: Offset { x: 2, y: 0 },
                        slot: Offset { x: 2, y: 1 },
                    },
                    ConnectorData {
                        port: Offset { x: 4, y: 1 },
                        slot: Offset { x: 3, y: 1 },
                    },
                ],
                outputs: &[ConnectorData {
                    port: Offset::NULL,
                    slot: Offset { x: 2, y: 3 },
                }],
            },
            Self::BigMerger => IoData {
                inputs: &[
                    ConnectorData {
                        port: Offset { x: 0, y: 0 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 1 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 2 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 3 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 4 },
                        slot: Offset::NULL,
                    },
                ],
                outputs: &[ConnectorData {
                    port: Offset { x: 0, y: 5 },
                    slot: Offset::NULL,
                }],
            },
            Self::BigSplitter => IoData {
                inputs: &[ConnectorData {
                    port: Offset { x: 0, y: 5 },
                    slot: Offset::NULL,
                }],
                outputs: &[
                    ConnectorData {
                        port: Offset { x: 0, y: 0 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 1 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 2 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 3 },
                        slot: Offset::NULL,
                    },
                    ConnectorData {
                        port: Offset { x: 0, y: 4 },
                        slot: Offset::NULL,
                    },
                ],
            },
        }
    }
}

impl From<&StructureDataFull> for StructureKind {
    fn from(value: &StructureDataFull) -> Self {
        use StructureDataFull::*;
        match value {
            AirPump { .. } => Self::AirPump,
            Refinery { .. } => Self::Refinery,
            Disharmonizer { .. } => Self::Disharmonizer,
            Unifier { .. } => Self::Unifier,
            SubdimensionalMarket { .. } => Self::SubdimensionalMarket,
            Splitter { .. } => Self::Splitter,
            Merger { .. } => Self::Merger,
            StorageVault { .. } => Self::StorageVault,
            AbysalDoor { .. } => Self::AbysalDoor,
            SingleStorage { .. } => Self::SingleStorage,
            Laboratory { .. } => Self::Laboratory,
            RitualInfuser { .. } => Self::RitualInfuser,
            BigMerger { .. } => Self::BigMerger,
            BigSplitter { .. } => Self::BigSplitter,
        }
    }
}

impl From<StructureKind> for StructureDataFull {
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
