use menu::menu::Menu;

use pancurses::Window;
use menu::mainmenu::MainMenu;

pub enum Menus {
    Main,
    SelectType,
}

impl Into<usize> for Menus {
    fn into(self) -> usize {
        match self {
            Menus::Main => 0,
            Menus::SelectType => 1,
        }
    }
}

pub struct MenuHandler {
    menus: Vec<Box<Menu>>,
    focused_menu: Option<usize>,
}

impl MenuHandler {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        let main_menu: MainMenu = MainMenu::new(max_x, max_y);

        MenuHandler {
            menus: vec![Box::new(main_menu)],
            focused_menu: Option::None,
        }
    }

    pub fn lose_focus(&mut self) {
        self.focused_menu = Option::None;
    }

    fn get_current_menu(&mut self) -> Option<&mut Menu> {
        if let Some(index) = self.focused_menu {
            Some(&mut (*self.menus[index]))
        } else {
            None
        }
    }

    fn current_menu_requires_focus(&mut self) -> bool {
        match self.get_current_menu() {
            Some(menu) => menu.requires_focus(),
            None => false,
        }
    }

    pub fn has_focus(&self) -> bool {
        match self.focused_menu {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn give_focus(&mut self, new_menu: Menus) {
        let index = new_menu.into();
        assert!(index < self.menus.len());
        self.focused_menu = Some(index);
    }

    pub fn update(&mut self) {
        match self.get_current_menu() {
            Some(menu) => if menu.requires_focus() {
                menu.give_focus();
            },
            None => return,
        }

        if !self.current_menu_requires_focus() {
            self.lose_focus();
        }
    }
}
