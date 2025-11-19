mod blueprint;

use std::{
    fmt::{self, Write},
    ops::Add,
    sync::Mutex,
};

use crate::structure::Structure;

static WORLD_COUNT: Mutex<usize> = Mutex::new(0);
fn new_world_id() -> WorldId {
    let mut guard = WORLD_COUNT
        .lock()
        .expect("Failed to lock global WORLD_COUNT");
    let id = *guard;
    *guard += 1;
    WorldId { id }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorldId {
    id: usize,
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

impl Add<Offset> for Position {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Offset> for PositionedStructure {
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

pub struct ConnectorOut {
    pub structure_id: StructureId,
    port: usize,
}

pub struct ConnectorIn {
    pub structure_id: StructureId,
    port: usize,
}

pub struct LogicalConnection {
    src: ConnectorOut,
    dst: ConnectorIn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DirectConnection {
    src: Position,
    dst: Position,
}

impl DirectConnection {
    fn export(&self, f: &mut impl Write, id: usize) -> fmt::Result {
        let (x1, y1) = self.src.world_coords();
        let (x2, y2) = self.dst.world_coords();
        writeln!(
            f,
            "{id}-struct=\"{{+point_a+:{{+x+:{x1}.0,+y+:{y1}.0,+type+:1}},+point_b+:{{+x+:{x2}.0,+y+:{y2}.0,+type+:0}}}}\"",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionedStructure {
    pub pos: Position,
    pub structure: Structure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructureId {
    world_id: WorldId,
    index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct World {
    world_id: WorldId,
    structures: Vec<PositionedStructure>,
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

    /// returns the index of the structure placed
    pub fn place_structure(&mut self, structure: Structure, x: i32, y: i32) -> StructureId {
        let id = StructureId {
            world_id: self.world_id,
            index: self.structures.len(),
        };
        self.structures.push(PositionedStructure {
            pos: Position { x, y },
            structure,
        });
        id
    }

    pub fn get_structure(&self, structure: StructureId) -> &PositionedStructure {
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
    pub fn connect(
        &mut self,
        source: StructureId,
        source_out_port: usize,
        destination: StructureId,
        destination_in_port: usize,
    ) {
        let src = self.get_structure(source);
        let src_data = &src.structure;
        let src_off = src_data
            .connectors()
            .outputs
            .get(source_out_port)
            .unwrap_or_else(|| {
                panic!(
                    "Source port does not exist.\n\
                    Tried to get {source:?} ({src_data:?}) output port #{source_out_port}"
                )
            });

        let dst = self.get_structure(destination);
        let dst_data = &dst.structure;
        let dst_off = dst_data
            .connectors()
            .inputs
            .get(destination_in_port)
            .unwrap_or_else(|| {
                panic!(
                    "Destination port does not exist.\n\
                    Tried to get {destination:?} ({dst_data:?}) output port #{destination_in_port}"
                )
            });

        let connection = DirectConnection {
            src: src.pos + *src_off,
            dst: dst.pos + *dst_off,
        };
        self.connections.push(connection);
    }

    pub fn export(&self, f: &mut impl Write) -> fmt::Result {
        writeln!(
            f,
            r#"[Intro]
-read="1.000000"
[Machines]"#
        )?;

        for (i, PositionedStructure { pos, structure }) in self.structures.iter().enumerate() {
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
