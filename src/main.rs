extern crate pancurses;

pub mod blocks;
pub mod application;

use pancurses::*;
use application::application::Application;

fn main() {

    // Creates the main window and initiating ncurses
    let stdscr: Window = initscr();

    // Launches the application
    let mut app: Application = Application::new(stdscr);
    app.launch();

    // Cleanup
    endwin();
}
