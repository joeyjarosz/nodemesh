use protocol::messages;

// NodeState behaves like a state machine.
pub struct NodeState {
    pub id: i64,
    cid: i64,
    pub children: Vec<Box<Node>>
}

impl NodeState {
    fn new() -> NodeState {
        NodeState{
            id: 0,
            cid: 0,
            children: vec![]
        }
    }
}

pub trait Node {
    fn get_state(&self) -> &NodeState;
    fn get_type(&self) -> messages::Node_NodeType;
}

pub struct Empty {
    state: NodeState
}

impl Empty {
    fn new() -> Empty {
        Empty{
            state: NodeState::new()
        }
    }
}

impl Node for Empty {
    fn get_state(&self) -> &NodeState {
        &self.state
    }

    fn get_type(&self) -> messages::Node_NodeType {
        messages::Node_NodeType::EMPTY
    }
}

pub struct Vertex {
    x: f64,
    y: f64,
    z: f64
}

pub struct Geometry {
    vertices: Vec<Vertex>,
    // faces are formed from indices of the vertices.
    faces: Vec<(usize, usize, usize)>
}

pub struct Mesh {
    state: NodeState,
    geometry: Geometry
}

impl Mesh {
    fn new(geometry: Geometry) -> Mesh {
        Mesh{
            state: NodeState::new(),
            geometry: geometry
        }
    }
}

impl Node for Mesh {
    fn get_state(&self) -> &NodeState {
        &self.state
    }

    fn get_type(&self) -> messages::Node_NodeType {
        messages::Node_NodeType::MESH
    }
}
