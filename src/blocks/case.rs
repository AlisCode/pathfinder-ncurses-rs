use pancurses::Window;

use pathfinding::node::Node;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeCase {
    Void,
    Wall,
    StartPoint,
    EndPoint,
}

impl TypeCase {
    pub fn from_u8(u: u8) -> Self {
        match u {
            49 => TypeCase::Void,
            50 => TypeCase::Wall,
            51 => TypeCase::EndPoint,
            52 => TypeCase::StartPoint,
            _ => unreachable!(),
        }
    }

    pub fn to_str(&self) -> &str {
        match *self {
            TypeCase::Void => "1",
            TypeCase::Wall => "2",
            TypeCase::EndPoint => "3",
            TypeCase::StartPoint => "4",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Case {
    x: i32,
    y: i32,
    flag: TypeCase,
}

impl Case {
    pub fn new(x: i32, y: i32, flag: TypeCase) -> Self {
        Case {
            x: x,
            y: y,
            flag: flag,
        }
    }

    pub fn get_x(&self) -> &i32 {
        &self.x
    }

    pub fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    pub fn get_y(&self) -> &i32 {
        &self.y
    }

    pub fn set_y(&mut self, new_y: i32) {
        self.y = new_y;
    }

    pub fn get_flag(&self) -> &TypeCase {
        &self.flag
    }

    pub fn set_flag(&mut self, flag: TypeCase) {
        self.flag = flag;
    }

    fn get_char_representation(&self) -> &'static str {
        match self.flag {
            TypeCase::Void => ".",
            TypeCase::Wall => "X",
            TypeCase::StartPoint => "S",
            TypeCase::EndPoint => "E",
        }
    }

    pub fn draw(&self, win: &Window) {
        win.mvprintw(self.y, self.x, self.get_char_representation());
    }

    pub fn get_flag_str(&self) -> &str {
        self.flag.to_str()
    }

    pub fn is_start_point(&self) -> bool {
        self.flag == TypeCase::StartPoint
    }

    pub fn is_end_point(&self) -> bool {
        self.flag == TypeCase::EndPoint
    }

    pub fn is_crossable(&self) -> bool {
        match self.flag {
            TypeCase::Wall => false,
            _ => true,
        }
    }

    pub fn create_node(&self) -> Node {
        Node::new(self.x, self.y)
    }
}