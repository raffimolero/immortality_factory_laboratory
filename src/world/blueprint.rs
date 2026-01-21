use super::*;

pub trait Machine {
    fn input(&self, port: usize) -> PortIn;
    fn output(&self, port: usize) -> PortOut;
}

pub trait Entity: Sized {
    fn get_world_id(&self) -> WorldId;
    fn _map_inside(&self, pasted_world: &PastedWorld) -> Self;

    fn inside(&self, pasted_world: &PastedWorld) -> Self {
        assert_eq!(
            self.get_world_id(),
            pasted_world.blueprint_id,
            "World IDs must match."
        );
        self._map_inside(pasted_world)
    }
}

impl Entity for Structure {
    fn get_world_id(&self) -> WorldId {
        self.world_id
    }

    fn _map_inside(&self, pasted_world: &PastedWorld) -> Self {
        Self {
            world_id: pasted_world.host_id,
            index: self.index + pasted_world.base_index,
            kind: self.kind,
        }
    }
}

impl Entity for PortIn {
    fn get_world_id(&self) -> WorldId {
        self.structure_id.world_id
    }

    fn _map_inside(&self, pasted_world: &PastedWorld) -> Self {
        Self {
            structure_id: self.structure_id._map_inside(pasted_world),
            index: self.index,
        }
    }
}

impl Entity for PortOut {
    fn get_world_id(&self) -> WorldId {
        self.structure_id.world_id
    }

    fn _map_inside(&self, pasted_world: &PastedWorld) -> Self {
        Self {
            structure_id: self.structure_id._map_inside(pasted_world),
            index: self.index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PastedWorld {
    blueprint_id: WorldId,
    host_id: WorldId,
    base_index: usize,
    offset: Offset,
}

impl PastedWorld {
    pub fn get<E: Entity>(&self, entity: E) -> E {
        entity.inside(self)
    }
}

// impl Entity for PastedWorld {
//     fn get_world_id(&self) -> WorldId {
//         self.host_id
//     }

//     // i hope this works
//     fn _map_inside(&self, pasted_world: &PastedWorld) -> Self {
//         Self {
//             blueprint_id: self.blueprint_id,
//             host_id: pasted_world.host_id,
//             base_index: self.base_index + pasted_world.base_index,
//             offset: self.offset + pasted_world.offset,
//         }
//     }
// }

impl Placeable for &World {
    type Id = PastedWorld;

    fn place_in(self, world: &mut World, x: Coord, y: Coord) -> PastedWorld {
        let base_index = world.structures.len();
        let offset = Offset { x, y };
        for &structure in &self.structures {
            let structure = structure + offset;
            world.assert_no_structure_collision(&structure);
            world.structures.push(structure);
        }
        PastedWorld {
            blueprint_id: self.world_id,
            host_id: world.world_id,
            base_index,
            offset,
        }
    }
}

impl World {
    /// for hardcore users
    pub fn stack_iter(
        &mut self,
        blueprint: &Self,
        x: Coord,
        y: Coord,
        dx: Coord,
        dy: Coord,
        count: usize,
    ) -> impl Iterator<Item = PastedWorld> {
        let delta = Offset { x: dx, y: dy };
        (0..count).scan(Position { x, y }, move |pos, _| {
            let building = self.place(blueprint, pos.x, pos.y);
            *pos = *pos + delta;
            Some(building)
        })
    }

    pub fn stack(
        &mut self,
        blueprint: &Self,
        x: Coord,
        y: Coord,
        dx: Coord,
        dy: Coord,
        count: usize,
    ) -> Vec<PastedWorld> {
        self.stack_iter(blueprint, x, y, dx, dy, count).collect()
    }
}

pub struct Blueprint {
    pub contents: World,
    pub size: Size,
    pub inputs: Vec<PortIn>,
    pub outputs: Vec<PortOut>,
}

impl HasSize for Blueprint {
    fn size(&self) -> Size {
        self.size
    }
}

impl Placeable for &Blueprint {
    type Id = PastedBlueprint;

    fn place_in(self, world: &mut World, x: Coord, y: Coord) -> Self::Id {
        let world = self.contents.place_in(world, x, y);
        PastedBlueprint {
            world,
            size: self.size,
            inputs: self
                .inputs
                .iter()
                .copied()
                .map(|p| p.inside(&world))
                .collect(),
            outputs: self
                .outputs
                .iter()
                .copied()
                .map(|p| p.inside(&world))
                .collect(),
        }
    }
}

pub struct PastedBlueprint {
    world: PastedWorld,
    size: Size,
    inputs: Vec<PortIn>,
    outputs: Vec<PortOut>,
}

impl HasSize for PastedBlueprint {
    fn size(&self) -> Size {
        self.size
    }
}

impl Machine for PastedBlueprint {
    fn input(&self, index: usize) -> PortIn {
        self.inputs[index]
    }

    fn output(&self, index: usize) -> PortOut {
        self.outputs[index]
    }
}

// impl Entity for PastedBlueprint {
//     fn get_world_id(&self) -> WorldId {
//         self.world.get_world_id()
//     }

//     fn _map_inside(&self, pasted_world: &PastedWorld) -> Self {
//         Self {
//             world: self.world._map_inside(pasted_world),
//             size: self.size,
//             inputs: self
//                 .inputs
//                 .iter()
//                 .copied()
//                 .map(|p| p._map_inside(pasted_world))
//                 .collect(),
//             outputs: self
//                 .outputs
//                 .iter()
//                 .copied()
//                 .map(|p| p._map_inside(pasted_world))
//                 .collect(),
//         }
//     }
// }
