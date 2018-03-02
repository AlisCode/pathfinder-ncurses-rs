use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Node {
    x: i32,
    y: i32,
    hcost: f32,
    gcost: f32,
    fcost: f32,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Self {
        Node {
            x,
            y,
            hcost: 0.0,
            gcost: 0.0,
            fcost: 0.0,
            parent: None,
        }
    }

    pub fn get_distance(&self, x_other: i32, y_other: i32) -> f32 {
        let dx_end: i32 = x_other - self.x;
        let dy_end: i32 = y_other - self.y;

        let total: f32 = (dx_end * dx_end + dy_end * dy_end) as f32;
        total.sqrt()
    }

    pub fn calculate_costs(&mut self, end: &Node) {

        match self.parent {
            Some(ref parent_rc) => {
                let parent = parent_rc.borrow();
                self.gcost = parent.gcost + 1.0;
            }
            None => (),
        }

        self.hcost = end.get_distance(self.x, self.y);
        self.fcost = self.hcost + self.gcost;

    }

    pub fn sort_by_fcost(&self, other: &Node) -> Ordering {
        other.compare_fcost(self.fcost)
    }

    pub fn compare_fcost(&self, fcost: f32) -> Ordering {
        if (self.fcost > fcost) {
            return Ordering::Less;
        } else if (self.fcost < fcost) {
            return Ordering::Greater;
        }
        Ordering::Equal
    }

    pub fn has_same_coordinates(&self, other: &Node) -> bool {
        other.compare_coordinates(self.x, self.y)
    }

    pub fn compare_coordinates(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}