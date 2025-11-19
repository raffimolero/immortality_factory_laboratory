use super::*;

pub trait Entity {
    fn get_world_id(&self) -> WorldId;
    fn inside_of(&self, pasted_world: &PastedWorld) -> Self;
}

impl Entity for StructureId {
    fn get_world_id(&self) -> WorldId {
        self.world_id
    }

    fn inside_of(&self, pasted_world: &PastedWorld) -> Self {
        Self {
            world_id: pasted_world.host_id,
            index: self.index + pasted_world.base_index,
            ..*self
        }
    }
}

impl Entity for StructureInput {
    fn get_world_id(&self) -> WorldId {
        self.structure_id.world_id
    }

    fn inside_of(&self, pasted_world: &PastedWorld) -> Self {
        Self {
            structure_id: self.structure_id.inside_of(pasted_world),
            offset: self.offset + pasted_world.offset,
        }
    }
}

impl Entity for StructureOutput {
    fn get_world_id(&self) -> WorldId {
        self.structure_id.world_id
    }

    fn inside_of(&self, pasted_world: &PastedWorld) -> Self {
        Self {
            structure_id: self.structure_id.inside_of(pasted_world),
            offset: self.offset + pasted_world.offset,
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
    pub fn get_in_host<E: Entity>(&self, entity: E) -> E {
        assert_eq!(
            entity.get_world_id(),
            self.blueprint_id,
            "World IDs must match."
        );
        entity.inside_of(self)
    }
}

impl World {
    pub fn paste(&mut self, blueprint: &Self, x: i32, y: i32) -> PastedWorld {
        let base_index = self.structures.len();
        let offset = Offset { x, y };
        self.structures.extend(
            blueprint
                .structures
                .iter()
                .cloned()
                .map(|structure| structure + offset),
        );
        self.connections.extend(
            blueprint
                .connections
                .iter()
                .copied()
                .map(|connection| connection + offset),
        );
        PastedWorld {
            blueprint_id: blueprint.world_id,
            host_id: self.world_id,
            base_index,
            offset,
        }
    }

    /// for hardcore users
    pub fn stack_iter(
        &mut self,
        blueprint: &Self,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
        count: usize,
    ) -> impl Iterator<Item = PastedWorld> {
        let delta = Offset { x: dx, y: dy };
        (0..count).scan(Position { x, y }, move |pos, _| {
            let building = self.paste(blueprint, pos.x, pos.y);
            *pos = *pos + delta;
            Some(building)
        })
    }

    pub fn stack(
        &mut self,
        blueprint: &Self,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
        count: usize,
    ) -> Vec<PastedWorld> {
        self.stack_iter(blueprint, x, y, dx, dy, count).collect()
    }
}
