use pancurses::Window;

pub enum TypeCase {
    Void,
    Wall,
    StartPoint,
    EndPoint,
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
        win.mvprintw(self.x, self.y, self.get_char_representation());
    }
}
