use pancurses::*;
use blocks::map::Map;

use std::ptr;
use std::sync::Arc;

pub struct Application {
    running: bool,
    stdscr: Window,
    map_window: Arc<Window>,
    map: Arc<Map>,
}

impl Application {
    pub fn new(stdscr: Window) -> Self {
        Application {
            running: false,
            stdscr: stdscr,
            map_window: ptr::null(),
            map: ptr::null(),
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
        self.map_window = Arc::new(map_window);

        // Creates the map itself
        let mut map: Map = Map::new();
        map.create_empty(10, 10);
        self.map = Arc::new(map);
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

        self.map.draw(&*self.map_window);
        self.stdscr.refresh();
        self.stdscr.getch();
    }
}