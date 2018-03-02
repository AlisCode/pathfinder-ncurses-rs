#[macro_use]
extern crate nom;
extern crate pancurses;
extern crate pathfinding;

use application::application::Application;
use pancurses::*;

pub mod blocks;
pub mod path;
pub mod menu;
pub mod application;

fn main() {

    // Creates the main window and initiating ncurses
    let stdscr: Window = initscr();

    // Launches the application
    let mut app: Application = Application::new(stdscr);
    app.launch();

    // Cleanup
    endwin();
}
