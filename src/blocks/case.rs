use pancurses::Window;

#[derive(PartialEq, Eq, Clone)]
pub enum TypeCase {
    Void,
    Wall,
    StartPoint,
    EndPoint,
}

impl TypeCase {
    pub fn from_u8(u: u8) -> Self {
        match u {
            48 => TypeCase::Void,
            49 => TypeCase::Wall,
            50 => TypeCase::EndPoint,
            51 => TypeCase::StartPoint,
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
}