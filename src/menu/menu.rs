use pancurses::{Input, Window, A_REVERSE};

pub struct MenuOption {
    pub name: &'static str,
    pub callback: &'static Fn() -> (),
}

pub trait Menu {
    fn draw(&self) {
        let selected = self.get_focused_option_index();
        let window = self.get_window();

        window.attroff(A_REVERSE);
        window.draw_box('|', '-');

        for (i, val) in self.get_list_options().iter().enumerate() {
            if i as i32 == selected {
                window.attron(A_REVERSE);
            } else {
                window.attroff(A_REVERSE);
            }

            if self.is_vertical() {
                window.mvaddstr(1, 0, val.name);
            } else {
                window.mvaddstr(0, 1, val.name);
            }
        }

        window.refresh();
    }

    fn update(&mut self) {
        match self.get_window().getch() {
            Some(input) => self.handle_input(input),
            None => return,
        }
    }

    fn give_focus(&mut self) {
        self.set_requires_focus(true);
        self.draw();
        self.update();
    }

    fn handle_input(&mut self, i: Input) {
        if self.is_vertical() {
            match i {
                Input::KeyDown => self.goto_next_option(),
                Input::KeyUp => self.goto_prev_option(),
                Input::KeyEnter => self.validate(),
                _ => return,
            }
        } else {
            match i {
                Input::KeyRight => self.goto_next_option(),
                Input::KeyLeft => self.goto_prev_option(),
                Input::KeyEnter => self.validate(),
                _ => return,
            }
        }
    }

    fn goto_next_option(&mut self) {
        let mut option: i32 = self.get_focused_option_index() + 1;

        if option > self.get_option_count() {
            option = 0;
        }

        self.set_focused_option_index(option);
    }

    fn goto_prev_option(&mut self) {
        let mut option: i32 = self.get_focused_option_index() + 1;

        if option < 0 {
            option = self.get_option_count() - 1;
        }

        self.set_focused_option_index(option);
    }

    fn validate(&mut self) {
        (self.get_list_options()[self.get_focused_option_index() as usize].callback)();
        self.set_requires_focus(false);
    }


    fn set_requires_focus(&mut self, focus: bool);
    fn requires_focus(&self) -> bool;
    fn is_vertical(&self) -> bool;

    fn get_option_count(&self) -> i32 {
        self.get_list_options().len() as i32
    }

    fn get_focused_option_index(&self) -> i32;
    fn set_focused_option_index(&mut self, index: i32);

    fn get_window(&self) -> &Window;

    fn get_list_options(&self) -> &Vec<MenuOption>;
    fn get_list_options_mutable(&mut self) -> &mut Vec<MenuOption>;

    fn register_option(&mut self, option: MenuOption) {
        self.get_list_options_mutable().push(option);
    }
}
