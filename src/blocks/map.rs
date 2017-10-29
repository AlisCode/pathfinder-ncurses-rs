use blocks::case::{Case, TypeCase};

use pancurses::Window;

pub struct Map {
    list_items: Vec<Case>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            list_items: Vec::new(),
        }
    }

    pub fn draw(&self, window: &Window) {
        window.clear();
        window.draw_box('|', '-');

        self.list_items.iter().for_each(|case| {
            case.draw(&window);
        });

        window.refresh();
    }

    pub fn create_empty(&mut self, size_x: i32, size_y: i32) {
        for x in 1..size_x {
            for y in 1..size_y {
                self.list_items.push(Case::new(x, y, TypeCase::Void));
            }
        }
    }
}
