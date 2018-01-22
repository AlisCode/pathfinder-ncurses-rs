use pancurses::{Window, newwin, Input};
use blocks::map::Map;
use blocks::case::TypeCase;

#[derive(PartialEq, Eq)]
pub enum MapWindowMode {
    Normal,
    Edit,
}

pub struct MapWindow {
    map: Map,
    window: Window,
    mode: MapWindowMode,
    type_case: TypeCase,
    edit_position_x: i32,
    edit_position_y: i32,
}

impl MapWindow {
    pub fn new(max_x: i32, max_y: i32, map: Map) -> Self {
        MapWindow {
            map,
            window: newwin(max_y, max_x, 0, 0),
            mode: MapWindowMode::Normal,
            type_case: TypeCase::Wall,
            edit_position_x: 1,
            edit_position_y: 1,
        }
    }

    pub fn draw(&self) {
        self.map.draw(
            &self.window,
            &self.edit_position_x,
            &self.edit_position_y,
            &self.mode,
        );
    }

    pub fn go_edit_mode(&mut self) {
        self.mode = MapWindowMode::Edit;
    }

    pub fn give_focus(&mut self) {
        self.update();
        self.draw();
        while self.mode == MapWindowMode::Edit {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
        match self.mode {
            // Temporary: bounds the edit to the window
            // TODO: handle this better once I get a camera of some sort :)
            MapWindowMode::Edit => {
                self.window.keypad(true);
                match self.window.getch() {
                    Some(Input::KeyLeft) if self.edit_position_x >= 2 => self.edit_position_x -= 1,
                    Some(Input::KeyRight) if self.edit_position_x < self.map.get_size_x() => {
                        self.edit_position_x += 1
                    }
                    Some(Input::KeyUp) if self.edit_position_y >= 2 => self.edit_position_y -= 1,
                    Some(Input::KeyDown) if self.edit_position_y < self.map.get_size_y() => {
                        self.edit_position_y += 1
                    } 
                    Some(Input::Character(' ')) => {
                        self.map.set_case(
                            &self.edit_position_x,
                            &self.edit_position_y,
                            &self.type_case,
                        );
                    }
                    Some(Input::Unknown(_)) => (),
                    Some(Input::Character(c)) if c == '\u{1b}' => self.mode = MapWindowMode::Normal,
                    _ => eprintln!("other"),
                }
            }
            MapWindowMode::Normal => (),
        }
    }

    pub fn save_map(&self) {
        match self.map.save() {
            _ => (),
        }
    }
}