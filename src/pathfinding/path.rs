use pathfinding::node::Node;

pub struct Path {
    list_nodes: Vec<Node>,
}

impl Path {
    pub fn new() -> Self {
        Path {
            list_nodes: Vec::new(),
        }
    }
}