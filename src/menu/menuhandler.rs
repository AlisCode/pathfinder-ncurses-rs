use pancurses::newwin;

use menu::menu::{Menu, MenuOption, MenuBuilder};
use menu::mainmenu::MainMenuMessage;
use menu::selecttypemenu::SelectTypeMenuMessage;

#[derive(Copy, Clone)]
pub enum Menus {
    Main,
    SelectType,
}

#[derive(Copy, Clone)]
pub enum MenusMessage {
    Main(MainMenuMessage),
    SelectType(SelectTypeMenuMessage),
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
    menus: Vec<Menu>,
    focused_menu: Option<usize>,
}

impl MenuHandler {
    pub fn new(max_x: i32, max_y: i32) -> Self {
        let main_menu = MenuBuilder::new()
            .set_vertical(false)
            .set_window(newwin(3, max_x, max_y - 3, 0))
            .with_option(MenuOption::new(
                "edit",
                MenusMessage::Main(MainMenuMessage::Edit),
            ))
            .with_option(MenuOption::new(
                "load",
                MenusMessage::Main(MainMenuMessage::Load),
            ))
            .with_option(MenuOption::new(
                "save",
                MenusMessage::Main(MainMenuMessage::Save),
            ))
            .with_option(MenuOption::new(
                "quit",
                MenusMessage::Main(MainMenuMessage::Quit),
            ))
            .build();

        MenuHandler {
            menus: vec![main_menu],
            focused_menu: Option::None,
        }
    }

    pub fn lose_focus(&mut self) {
        self.focused_menu = Option::None;
    }

    fn get_current_menu(&mut self) -> Option<&mut Menu> {
        if let Some(index) = self.focused_menu {
            Some(&mut self.menus[index])
        } else {
            None
        }
    }

    pub fn give_focus(&mut self, new_menu: Menus) {
        let index = new_menu.into();
        assert!(index < self.menus.len());
        self.focused_menu = Some(index);
        if let Some(menu) = self.get_current_menu() {
            menu.give_focus();
        }
    }

    pub fn has_focus(&self) -> bool {
        if let Some(_) = self.focused_menu {
            return true;
        }

        false
    }

    pub fn update(&mut self) -> Option<MenusMessage> {
        let mut loses_focus: bool = false;
        let mut opt_message: Option<MenusMessage> = None;


        match self.get_current_menu() {
            Some(menu) => {
                if menu.requires_focus() {
                    opt_message = menu.tick();
                } else {
                    loses_focus = true;
                }
            }
            None => (),
        }


        if loses_focus {
            self.lose_focus();
        }

        opt_message
    }
}
