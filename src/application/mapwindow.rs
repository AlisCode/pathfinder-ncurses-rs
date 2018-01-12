use pancurses::{Window, newwin};
use blocks::map;

pub struct MapWindow {
    map: Map,
    window: Window,
}

impl MapWindow {
    pub fn new(max_x: i32, max_y: i32) {
        MapWindow {
            map: Map::new(),
            window: newwin(max_y, max_x, 0, 0),
        }
    }
}