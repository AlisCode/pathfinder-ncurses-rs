use pancurses::{newwin, Window};
use menu::menu::*;

pub struct MainMenu {
    options: Vec<MenuOption>,
    window: Window,
    selected_index: i32,
    validated: bool,
}

fn test_callback() {
    eprintln!("Test!");
}

impl MainMenu {
    pub fn option_edit(&self) {}

    pub fn new(max_x: i32, max_y: i32) -> Self {
        let option_edit = MenuOption {
            name: "edit",
            callback: &test_callback,
        };

        MainMenu {
            options: vec![option_edit],
            window: newwin(3, max_x, max_y - 3, 0),
            selected_index: 0,
            validated: true,
        }
    }
}

impl Menu for MainMenu {
    fn is_vertical(&self) -> bool {
        false
    }

    fn requires_focus(&self) -> bool {
        self.validated
    }

    fn set_requires_focus(&mut self, focus: bool) {
        self.validated = focus;
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
