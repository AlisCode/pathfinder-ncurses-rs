use std::num::sqrt;
use std::ptr::null;
use pancurses::Window;

enum NodeDirection {
    PathRight,
    PathLeft,
    PathDown,
    PathUp,
}

pub struct Node {
    x: i32,
    y: i32,
    hcost: i32,
    gcost: i32,
    fcost: i32,
    flag: Option<NodeDirection>,
    parent: Arc<Node>,
}

impl Node {
    pub fn new(x: i32, y: i32) {
        Node {
            x: x,
            y: y,
            hcost: 0,
            gcost: 0,
            fcost: 0,
            flag: None,
            parent: ptr::null(),
        }
    }

    pub fn draw(&self, win: &Window) {}

    pub fn get_x(&self) {
        &self.x
    }

    pub fn get_y(&self) {
        &self.y
    }

    pub fn set_parent()

    pub fn calculate_costs(&mut self, end: &Node) {
        
        let dx_end: i32 = end.get_x() - self.x;
        let dy_end: i32 = end.get_y() - self.y;

        self.hcost = sqrt(dx_end*dx_end + dy_end*dy_end);
        // TODO: calculate gcost based on parent self.gcost = 
        self.fcost = self.hcost + self.gcost;

    }

    pub fn set_flag(&mut self, next_node: &Node) {

        let dx: i32 = self.x - next_node.get_x();
        let dy: i32 = self.y - next_node.get_y();

        if dx > 0 {
            self.flag = NodeDirection::PathLeft;
        } else if dx < 0 {
            self.flag = NodeDirection::PathRight;
        } else if dy > 0 {
            self.flag = NodeDirection::PathUp;
        } else if dy < 0 {
            self.flag = NodeDirection::PathDown;
        }

    }

    fn get_char_representation(&self) -> &str {
        match self.flag.unwrap() {
            NodeDirection::PathRight => ">",
            NodeDirection::PathLeft => "<",
            NodeDirection::PathUp => "^",
            NodeDirection::PathDown => "v",
            _ => "?",
        }
    }
}