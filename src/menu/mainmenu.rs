use pancurses::{newwin, Window};
use menu::menu::*;

pub struct MainMenu {
    options: Vec<MenuOption>,
    window: Window,
    selected_index: i32,
}

impl MainMenu {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        MainMenu {
            options: Vec::new(),
            window: newwin(3, max_x, max_y - 3, 0),
            selected_index: 0,
        }
    }
}

impl Menu for MainMenu {
    fn is_vertical(&self) -> bool {
        false
    }

    fn get_focused_option_index(&self) -> i32 {
        self.selected_index.clone()
    }

    fn set_focused_option_index(&mut self, new_index: i32) {
        self.selected_index = new_index;
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn get_list_options(&self) -> &Vec<MenuOption> {
        &self.options
    }

    fn get_list_options_mutable(&mut self) -> &mut Vec<MenuOption> {
        &mut self.options
    }
}
