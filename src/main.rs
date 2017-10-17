extern crate pancurses;

use pancurses::*;

pub mod blocks;

fn main() {
    
    let stdscr: Window = initscr();
    noecho();
    cbreak();
    curs_set(0);
    
    let max_x: i32 = stdscr.get_max_x();
    let max_y: i32 = stdscr.get_max_y();

    endwin();
}
