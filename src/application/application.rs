use pancurses::*;
use blocks::map::Map;

pub struct Application {
    running: bool,
    stdscr: Window,
    map_window: Option<Window>,
    map: Option<Map>,
}

impl Application {
    pub fn new(stdscr: Window) -> Self {
        Application {
            running: false,
            stdscr: stdscr,
            map_window: None,
            map: None,
        }
    }

    fn setup(&mut self) {
        self.stdscr.nodelay(false);
        noecho();
        cbreak();
        curs_set(0);
        self.stdscr.refresh();

        // Stores the max X and Y of the window
        let max_x: i32 = self.stdscr.get_max_x();
        let max_y: i32 = self.stdscr.get_max_y();

        // Creates the map window
        let map_window: Window = newwin(max_y, max_x, 0, 0);
        self.map_window = Some(map_window);

        // Creates the map itself
        let mut map: Map = Map::new();
        map.create_empty(10, 10);
        self.map = Some(map);
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
        self.map
            .take()
            .unwrap()
            .draw(&self.map_window.take().unwrap());

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
            _ => return,
        }
    }

    // Handle the given input if it is a char
    fn handle_input_char(&mut self, c: char) {
        match c {
            'm' => return,
            _ => return,
        }

    }
}
