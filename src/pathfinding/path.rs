use pathfinder_ncurses_rs::pathfinding::node::Node;
use pancurses::Window;

pub struct Path {
    list_nodes: Vec<Node>;
}

impl Path {

    pub fn new() -> Self {
        Path {
            list_nodes: Vec::new(),
        }
    }

    pub fn from_last_node(&mut self, node: &Node) {
        //TODO: construct path from last node
    }

    pub fn draw(&mut self, win: &Window) {
        //TODO: Draw the path on the given window
    }

}