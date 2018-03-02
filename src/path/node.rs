#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    x: i32,
    y: i32,
}

impl Node {
    pub fn new(x: i32, y: i32) -> Self {
        Node {
            x,
            y,
        }
    }

    pub fn get_distance(&self, other: &Node) -> usize {
        other.calc_distance(self.x, self.y)
    }

    pub fn calc_distance(&self, x_other: i32, y_other: i32) -> usize {
        let dx_end: i32 = x_other - self.x;
        let dy_end: i32 = y_other - self.y;

        let total: f32 = (dx_end * dx_end + dy_end * dy_end) as f32;
        total.sqrt().ceil() as usize
    }

    pub fn is_neighbor(&self, other: &Node) -> bool {
        other.calc_distance(self.x, self.y) == 1
    }

    pub fn get_neighbors(&self, list: &Vec<Node>) -> Vec<(Node, usize)> {
        list.iter().filter_map(|a| {
            if self.is_neighbor(a) {
                let returned = (*a).clone();
                return Some((returned, 1));
            }
            None
        }).collect()
    }

    pub fn get_coordinates(&self) -> (&i32, &i32) {
        (&self.x, &self.y)
    }
}