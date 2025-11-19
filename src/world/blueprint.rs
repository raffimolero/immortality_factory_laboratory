use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PastedWorld {
    blueprint_id: WorldId,
    host_id: WorldId,
    base_index: usize,
}

impl PastedWorld {
    pub fn get_in_host(&self, structure: StructureId) -> StructureId {
        assert_eq!(
            structure.world_id, self.blueprint_id,
            "World IDs must match."
        );
        StructureId {
            world_id: self.host_id,
            index: structure.index + self.base_index,
            ..structure
        }
    }
}

impl World {
    pub fn paste(&mut self, blueprint: &Self, x: i32, y: i32) -> PastedWorld {
        let base_index = self.structures.len();
        self.structures.extend(
            blueprint
                .structures
                .iter()
                .cloned()
                .map(|pos_struct| pos_struct + Offset { x, y }),
        );
        self.connections.extend(
            blueprint
                .connections
                .iter()
                .map(|conn| *conn + Offset { x, y }),
        );
        PastedWorld {
            blueprint_id: blueprint.world_id,
            host_id: self.world_id,
            base_index,
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
