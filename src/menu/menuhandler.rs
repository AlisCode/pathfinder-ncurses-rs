use menu::menu::Menu;

use std::rc::Rc;

pub struct MenuHandler {
    focused_menu: Option<Rc<Menu>>,
}

impl MenuHandler {
    pub fn new() -> Self {
        MenuHandler {
            focused_menu: Option::None,
        }
    }

    pub fn lose_focus(&mut self) {
        self.focused_menu = None;
    }

    pub fn give_focus(&mut self, new_menu: Rc<Menu>) {
        self.focused_menu = Some(new_menu);
    }

    pub fn update(&mut self) {
        match self.focused_menu {
            Some(menu) => {
                menu.update();
            }
            None => return,
        }
    }
}
