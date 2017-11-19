use pancurses::{Window, A_REVERSE};

pub struct MenuOption {
    name: &'static str,
    callback: &'static Fn() -> (),
}

pub trait Menu {
    fn draw(&self) {

        let selected = self.get_focused_option_index();
        let window = self.get_window();
        window.draw_box('|', '-');

        for (i, val) in self.get_list_options().iter().enumerate() {
            if i as i32 == selected {
                window.attron(A_REVERSE);
            } else {
                window.attroff(A_REVERSE);
            }

            window.mvaddstr(0, 1, val.name);
        }

        window.refresh();
    }

    fn get_focused_option_index(&self) -> i32;
    fn set_focused_option_index(&mut self, index: i32);

    fn get_window(&self) -> &Window;
    fn get_list_options(&self) -> &Vec<MenuOption>;
    fn get_list_options_mutable(&self) -> &mut Vec<MenuOption>;

    fn register_option(&mut self, option: MenuOption) {
        self.get_list_options_mutable().push(option);
    }
}
