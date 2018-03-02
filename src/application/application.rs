use application::mapwindow::MapWindow;
use blocks::map::Map;
use menu::mainmenu::MainMenuMessage;
use menu::menuhandler::{MenuHandler, Menus, MenusMessage};
use menu::selecttypemenu::SelectTypeMenuMessage;
use pancurses::*;

pub struct Application {
    running: bool,
    stdscr: Window,
    menu_handler: MenuHandler,
    map_window: MapWindow,
}

impl Application {
    pub fn new(stdscr: Window) -> Self {
        // Stores the max X and Y of the window
        let max_x: i32 = stdscr.get_max_x();
        let max_y: i32 = stdscr.get_max_y();

        // Creates the map itself
        let mut map: Map = Map::new();
        map.create_empty(10, 10);

        // Creates the map window
        let map_window: MapWindow = MapWindow::new(max_x, max_y, map);

        // Returns the application struct
        Application {
            running: false,
            stdscr,
            map_window,
            menu_handler: MenuHandler::new(max_x, max_y),
        }
    }

    /// Setups the application
    fn setup(&mut self) {
        self.stdscr.nodelay(false);
        noecho();
        cbreak();
        curs_set(0);
        self.stdscr.refresh();
    }

    /// Launches the application
    pub fn launch(&mut self) {
        self.setup();

        self.running = true;

        while self.running {
            self.update();
        }
    }

    // Update the application
    fn update(&mut self) {
        self.map_window.draw();
        if !self.map_window.needs_menu_type_focusing() {
            self.map_window.give_focus(&mut self.menu_handler);
        }

        let menu_has_focus: bool = self.menu_handler.has_focus();
        if menu_has_focus {
            let message = self.menu_handler.update();
            self.handle_message(message);
        } else {
            self.stdscr.refresh();
            match self.stdscr.getch() {
                Some(input) => self.handle_input(input),
                None => return,
            }
        }
    }

    // Handle the given input
    fn handle_input(&mut self, input: Input) {
        match input {
            Input::Character(c) => self.handle_input_char(c),
            Input::Unknown(i) => self.handle_input_unknown(i),
            _ => return,
        }
    }

    // Handle the given input if it is a char
    fn handle_input_char(&mut self, c: char) {
        match c {
            'm' => self.menu_handler.give_focus(Menus::Main), // M key, activates the main menu
            'q' => self.quit(),                               // Q key, quits the application
            _ => return,
        }
    }

    // Handle the given input if it is unknown
    fn handle_input_unknown(&mut self, i: i32) {
        match i {
            27 => self.quit(), // ESC Key, quits the application
            _ => return,
        }
    }

    /// Handles the given message
    fn handle_message(&mut self, message: Option<MenusMessage>) {
        match message {
            Some(MenusMessage::Main(msg)) => self.handle_menu_main_message(msg),
            Some(MenusMessage::SelectType(msg)) => self.handle_menu_select_type_message(msg),
            None => (),
        }
    }

    /// Handles the given message if it came from the main menu
    fn handle_menu_main_message(&mut self, message: MainMenuMessage) {
        match message {
            MainMenuMessage::Edit => self.map_window.go_edit_mode(),
            MainMenuMessage::Load => (),
            MainMenuMessage::Save => self.map_window.save_map(),
            MainMenuMessage::Quit => self.quit(),
            MainMenuMessage::Solve => self.map_window.solve_map(),
        }
    }

    /// Handles the given message if it came from the select type menu
    fn handle_menu_select_type_message(&mut self, message: SelectTypeMenuMessage) {
        // This is handled as a match, because I might need to add other types of messages
        // Plus, it's convenient. 
        match message {
            SelectTypeMenuMessage::ChangeTypeCase(type_case) => {
                self.map_window.set_type_case(type_case);
                self.map_window.set_menu_type_focus(false);
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}