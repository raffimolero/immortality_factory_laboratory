pub mod blueprint;

use std::{
    io::{self, Write},
    num::NonZeroU32,
    ops::Add,
    sync::Mutex,
};

use crate::prelude::*;
// use super::structure::{StructureData, StructureKind};

type ID = NonZeroU32;
static WORLD_COUNT: Mutex<ID> = Mutex::new(NonZeroU32::new(1).unwrap());
fn new_world_id() -> WorldId {
    // could be a UUID instead of an incrementing count
    let mut guard = WORLD_COUNT
        .lock()
        .expect("Failed to lock global WORLD_COUNT");
    let id = *guard;
    *guard = guard
        .checked_add(1)
        .expect("How? You have more than u32::MAX worlds?");
    WorldId { id }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldId {
    id: ID,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn world_x(&self) -> i32 {
        self.x * 22
    }

    fn world_y(&self) -> i32 {
        self.y * 22
    }

    fn world_coords(&self) -> (i32, i32) {
        (self.world_x(), self.world_y())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Offset {
    pub const NULL: Self = Self { x: -1, y: -1 };

    pub fn non_null(&self) -> bool {
        *self != Self::NULL
    }
}

impl Add<Self> for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Offset> for PositionedStructureData {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            pos: self.pos + rhs,
            ..self
        }
    }
}

impl Add<Offset> for DirectConnection {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            src: self.src + rhs,
            dst: self.dst + rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DirectConnection {
    src: Position,
    dst: Position,
}

impl DirectConnection {
    fn export(&self, f: &mut impl Write, id: usize) -> io::Result<()> {
        let (x1, y1) = self.src.world_coords();
        let (x2, y2) = self.dst.world_coords();
        writeln!(
            f,
            "{id}-struct=\"{{+point_a+:{{+x+:{x1}.0,+y+:{y1}.0,+type+:1}},+point_b+:{{+x+:{x2}.0,+y+:{y2}.0,+type+:0}}}}\"",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionedStructureData {
    pub pos: Position,
    pub structure: StructureData,
}

/// technically only the index is necessary. the rest are for debug assertions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Structure {
    world_id: WorldId,
    index: usize,
    kind: StructureKind,
}

impl HasSize for Structure {
    fn size(&self) -> Size {
        self.kind.size()
    }
}

impl Structure {
    pub fn input(self, port: usize) -> PortIn {
        let structure = &self.kind;
        let offset = structure
            .connectors()
            .inputs
            .get(port)
            .copied()
            .filter(Offset::non_null)
            .unwrap_or_else(|| {
                panic!("Tried to get {structure:?} input port #{port}, does not exist.")
            });
        PortIn {
            structure_id: self,
            offset,
        }
    }

    pub fn output(self, port: usize) -> PortOut {
        let structure = &self.kind;
        let offset = structure
            .connectors()
            .outputs
            .get(port)
            .copied()
            .filter(Offset::non_null)
            .unwrap_or_else(|| {
                panic!("Tried to get {structure:?} output port #{port}, does not exist.")
            });
        PortOut {
            structure_id: self,
            offset,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortIn {
    structure_id: Structure,
    offset: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PortOut {
    structure_id: Structure,
    offset: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl Size {
    pub const NULL: Self = Self { w: -1, h: -1 };

    pub fn non_null(&self) -> bool {
        *self != Self::NULL
    }
}

impl Add<Self> for Size {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w + rhs.w,
            h: self.h + rhs.h,
        }
    }
}

pub trait HasSize {
    fn size(&self) -> Size;

    fn width(&self) -> i32 {
        self.size().w
    }

    fn height(&self) -> i32 {
        self.size().h
    }
}

pub trait Placeable {
    type Id;

    fn place_in(self, world: &mut World, x: i32, y: i32) -> Self::Id;
}

impl HasSize for StructureData {
    fn size(&self) -> Size {
        self.kind().size()
    }
}

impl Placeable for StructureData {
    type Id = Structure;

    fn place_in(self, world: &mut World, x: i32, y: i32) -> Self::Id {
        let id = Structure {
            world_id: world.world_id,
            index: world.structures.len(),
            kind: self.kind(),
        };
        world.structures.push(PositionedStructureData {
            pos: Position { x, y },
            structure: self,
        });
        id
    }
}

impl Placeable for StructureKind {
    type Id = Structure;

    fn place_in(self, world: &mut World, x: i32, y: i32) -> Self::Id {
        StructureData::from(self).place_in(world, x, y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct World {
    world_id: WorldId,
    structures: Vec<PositionedStructureData>,
    connections: Vec<DirectConnection>,
}

impl World {
    pub fn new() -> Self {
        Self {
            world_id: new_world_id(),
            structures: vec![],
            connections: vec![],
        }
    }

    // TODO: collision detection
    pub fn place<P: Placeable>(&mut self, object: P, x: i32, y: i32) -> P::Id {
        object.place_in(self, x, y)
    }

    pub fn get_structure(&self, structure: Structure) -> &PositionedStructureData {
        assert_eq!(structure.world_id, self.world_id, "World IDs must match.");
        self.structures.get(structure.index).unwrap_or_else(|| {
            panic!(
                "Source structure does not exist.\n\
                Tried to get {structure:?} but only {} structures exist.",
                self.structures.len()
            )
        })
    }

    /// panics if you mess anything up lmao
    pub fn connect(&mut self, source: PortOut, destination: PortIn) {
        let src = self.get_structure(source.structure_id);
        let dst = self.get_structure(destination.structure_id);

        let connection = DirectConnection {
            src: src.pos + source.offset,
            dst: dst.pos + destination.offset,
        };
        self.connections.push(connection);
    }

    pub fn export(&self, f: &mut impl Write) -> io::Result<()> {
        writeln!(
            f,
            r#"[Intro]
-read="1.000000"
[Machines]"#
        )?;

        for (i, PositionedStructureData { pos, structure }) in self.structures.iter().enumerate() {
            structure.export(f, i, pos.x, pos.y)?;
        }
        let structure_count = self.structures.len();
        writeln!(
            f,
            r#"total="{structure_count}.000000"
[Machine-Type]
0-cur-input="0.000000"
1-cur-input="0.000000"
2-cur-input="0.000000"
3-cur-input="0.000000"
4-cur-input="0.000000"
5-cur-input="0.000000"
6-cur-input="0.000000"
7-cur-input="0.000000"
8-cur-input="0.000000"
9-cur-input="0.000000"
10-cur-input="0.000000"
11-cur-input="0.000000"
12-cur-input="0.000000"
13-cur-input="0.000000"
[Connections]"#
        )?;

        for (i, connection) in self.connections.iter().enumerate() {
            connection.export(f, i)?;
        }
        let connection_count = self.connections.len();
        writeln!(
            f,
            r#"total="{connection_count}.000000"
[Research]
0-name="Start Factory"
0-researched="1.000000"
0-cost_input="4.000000"
1-name="Pump Speed"
1-researched="1.000000"
1-cost_input="8.000000"
2-name="Research Speed"
2-researched="1.000000"
2-cost_input="16.000000"
3-name="Refine Resource"
3-researched="1.000000"
3-cost_input="4.000000"
4-name="Automation"
4-researched="1.000000"
4-cost_input="2.000000"
5-name="Splitter/Merger"
5-researched="1.000000"
5-cost_input="6.000000"
6-name="Destroy Stuff"
6-researched="1.000000"
6-cost_input="16.000000"
7-name="Portalisation"
7-researched="1.000000"
7-cost_input="32.000000"
8-name="Bigger Storage"
8-researched="1.000000"
8-cost_input="16.000000"
9-name="Storage Use"
9-researched="1.000000"
9-cost_input="10.000000"
10-name="Combine Stuff"
10-researched="1.000000"
10-cost_input="80.000000"
11-name="Crystal-1"
11-researched="1.000000"
11-cost_input="24.000000"
12-name="Better Gems"
12-researched="1.000000"
12-cost_input="36.000000"
13-name="Destruction+"
13-researched="1.000000"
13-cost_input="48.000000"
14-name="RefineSpeed+"
14-researched="1.000000"
14-cost_input="130.000000"
15-name="BetterPlate"
15-researched="1.000000"
15-cost_input="24.000000"
16-name="Sell Stuff"
16-researched="1.000000"
16-cost_input="90.000000"
17-name="Lab Speed+"
17-researched="1.000000"
17-cost_input="30.000000"
18-name="Bigger Sp/Me"
18-researched="1.000000"
18-cost_input="60.000000"
19-name="Better Shard"
19-researched="1.000000"
19-cost_input="90.000000"
20-name="Better Metal"
20-researched="1.000000"
20-cost_input="90.000000"
21-name="Easier Gems"
21-researched="1.000000"
21-cost_input="50.000000"
22-name="Best Metal"
22-researched="1.000000"
22-cost_input="80.000000"
23-name="Infuse Metal"
23-researched="1.000000"
23-cost_input="120.000000"
24-name="The Chassis"
24-researched="1.000000"
24-cost_input="100.000000"
25-name="ORB-ORB-ORB"
25-researched="1.000000"
25-cost_input="90.000000"
26-name="ORB POWER"
26-researched="1.000000"
26-cost_input="80.000000"
27-name="Pure Energy"
27-researched="1.000000"
27-cost_input="120.000000"
28-name="Dark Arts"
28-researched="1.000000"
28-cost_input="300.000000"
29-name="Curse Recipe"
29-researched="1.000000"
29-cost_input="50.000000"
30-name="Air Recipe"
30-researched="1.000000"
30-cost_input="120.000000"
31-name="Fleshy Stuff"
31-researched="1.000000"
31-cost_input="90.000000"
32-name="Flesh Infusion"
32-researched="1.000000"
32-cost_input="80.000000"
33-name="Easier Flesh"
33-researched="1.000000"
33-cost_input="30.000000"
34-name="Darker Arts"
34-researched="1.000000"
34-cost_input="40.000000"
35-name="Thought Cores"
35-researched="1.000000"
35-cost_input="50.000000"
36-name="Soul Magic"
36-researched="1.000000"
36-cost_input="70.000000"
37-name="The Ritual"
37-researched="1.000000"
37-cost_input="800.000000"
38-name="Immortality"
38-researched="1.000000"
38-cost_input="1.000000"
39-name="INSANITY"
39-researched="0.000000"
39-cost_input="0.000000"
[Special Unlocks]
0-unlocked="1.000000"
1-unlocked="1.000000"
2-unlocked="1.000000"
[Hand]
-type="-1.000000"
-value="-1.000000"
[Final]
0-value="0.000000"
1-value="0.000000"
2-value="0.000000"
[Game]
-finished="1.000000""#
        )
    }
}
