pub mod item;
pub mod structure;
pub mod world;

pub mod prelude {
    pub use super::{
        item::Item::{self, *},
        structure::{StructureKind::*, StructureWithData},
        world::{blueprint::Entity, StructureId, World},
    };
}
