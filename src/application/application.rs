use pancurses::*;
use blocks::map::Map;

pub struct Application {
    running: bool,
    stdscr: Window,
    map_window: Window,
    map: Map,
}

impl Application {
    pub fn new(stdscr: Window) -> Self {
        // Stores the max X and Y of the window
        let max_x: i32 = stdscr.get_max_x();
        let max_y: i32 = stdscr.get_max_y();

        // Creates the map window
        let map_window: Window = newwin(max_y, max_x, 0, 0);

        // Creates the map itself
        let mut map: Map = Map::new();
        map.create_empty(10, 10);

        Application {
            running: false,
            stdscr: stdscr,
            map_window: map_window,
            map: map,
        }
    }

    fn setup(&mut self) {
        self.stdscr.nodelay(false);
        noecho();
        cbreak();
        curs_set(0);
        self.stdscr.refresh();
    }

    pub fn launch(&mut self) {
        self.setup();

        self.running = true;

        while self.running {
            self.update();
        }
    }

    // Update the application
    fn update(&mut self) {
        self.map.draw(&self.map_window);

        self.stdscr.refresh();

        match self.stdscr.getch() {
            Some(input) => self.handle_input(input),
            None => return,
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
            'm' => return,      // M key
            'q' => self.quit(), // Q key
            _ => return,
        }
    }

    // Handle the given input if it is unknown
    fn handle_input_unknown(&mut self, i: i32) {
        match i {
            27 => self.quit(), // ESC Key
            _ => return,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
