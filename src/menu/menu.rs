use pancurses::{Input, Window, A_REVERSE};
use menu::menuhandler::MenusMessage;

pub struct MenuOption {
    pub name: String,
    pub message: MenusMessage,
}

impl MenuOption {
    pub fn new(option_name: &str, mess: MenusMessage) -> Self {
        MenuOption {
            name: option_name.into(),
            message: mess,
        }
    }
}

pub struct Menu {
    options: Vec<MenuOption>,
    selected_index: usize,
    window: Window,
    vertical: bool,
    requires_focus: bool,
}

impl Menu {
    fn draw(&self) {
        // Places the cursor adequately
        self.window.clear();
        self.window.attroff(A_REVERSE);
        self.window.draw_box('|', '-');

        if !self.vertical {
            self.window.mv(1, 1);
        }
        

        // Displays all the options of the menu
        self.options.iter().enumerate().for_each(
            |(index, curr_option)| {

                // Sets the highlighted option 
                if index == self.selected_index {
                    self.window.attron(A_REVERSE);
                } else {
                    self.window.attroff(A_REVERSE);
                }

                let (mut y_cursor, mut x_cursor) = self.window.get_cur_yx();

                // Moves the cursor to display the next option
                if self.vertical {
                    y_cursor += 1;
                    x_cursor = 1;
                } else {
                    x_cursor += 1;
                }
                self.window.mv(y_cursor, x_cursor);

                // Effectively displays the option
                self.window.printw(&curr_option.name);
            },
        );

        // Refreshes the window so all of what we just drawn now gets displayed on the screen
        self.window.refresh();
    }

    fn update(&mut self) -> Option<MenusMessage> {
        self.window.keypad(true);
        match self.window.getch() {
            Some(input) => self.handle_input(input),
            None => None,
        }
    }

    pub fn give_focus(&mut self) {
        self.requires_focus = true;
    }

    pub fn tick(&mut self) -> Option<MenusMessage> {
        self.draw();
        self.update()
    }

    fn handle_input(&mut self, i: Input) -> Option<MenusMessage> {
        if self.vertical {
            match i {
                Input::KeyDown => {
                    self.next_option();
                    return None;
                }
                Input::KeyUp => {
                    self.prev_option();
                    return None;
                }
                Input::Character(c) if c == '\n' => return self.validate(),
                Input::Character(c) if c == '\u{1b}' => {
                    self.quit();
                    return None;
                }
                _ => return None,
            }
        } else {
            match i {
                Input::KeyRight => {
                    self.next_option();
                    return None;
                }
                Input::KeyLeft => {
                    self.prev_option();
                    return None;
                }
                Input::Character(c) if c == '\n' => return self.validate(),
                Input::Character(c) if c == '\u{1b}' => {
                    self.quit();
                    return None;
                }
                _ => return None,
            }
        }
    }

    fn next_option(&mut self) {
        self.selected_index += 1;
        if self.selected_index >= self.options.len() {
            self.selected_index = 0;
        }
    }

    fn prev_option(&mut self) {
        if let Some(new_index) = self.selected_index.checked_sub(1) {
            self.selected_index = new_index;
        } else {
            self.selected_index = self.options.len() - 1;
        }
    }

    fn validate(&mut self) -> Option<MenusMessage> {
        self.requires_focus = false;
        Some(self.options[self.selected_index].message)
    }

    fn quit(&mut self) {
        self.requires_focus = false;
    }

    pub fn requires_focus(&self) -> bool {
        self.requires_focus
    }
}

pub struct MenuBuilder {
    options: Vec<MenuOption>,
    selected_index: usize,
    window: Option<Window>,
    vertical: bool,
}

impl MenuBuilder {
    pub fn new() -> Self {
        MenuBuilder {
            options: Vec::new(),
            selected_index: 0,
            window: None,
            vertical: false,
        }
    }

    pub fn with_option(mut self, opt: MenuOption) -> Self {
        self.options.push(opt);
        self
    }
    pub fn set_vertical(mut self, is_vertical: bool) -> Self {
        self.vertical = is_vertical;
        self
    }
    pub fn set_start_index(mut self, index: usize) -> Self {
        self.selected_index = index;
        self
    }

    pub fn set_window(mut self, window: Window) -> Self {
        self.window = Some(window);
        self
    }

    pub fn build(self) -> Menu {
        Menu {
            options: self.options,
            selected_index: self.selected_index,
            window: self.window.unwrap(),
            vertical: self.vertical,
            requires_focus: false,
        }
    }
}
