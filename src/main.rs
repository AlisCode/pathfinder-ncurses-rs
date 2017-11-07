#[macro_use]
extern crate nom;
extern crate pancurses;

pub mod blocks;
pub mod application;

use pancurses::*;
use application::application::Application;

use blocks::mapparser::load_map;

fn main() {
    let test = load_map("test.map".into());
    println!("{:?}", test);

    /*
    // Creates the main window and initiating ncurses
    let stdscr: Window = initscr();

    // Launches the application
    let mut app: Application = Application::new(stdscr);
    app.launch();

    // Cleanup
    endwin();
    */
}
