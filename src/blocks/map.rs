use blocks::case::{Case, TypeCase};

use pancurses::{Window, A_REVERSE};
use application::mapwindow::MapWindowMode;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub struct Map {
    name: String,
    list_items: Vec<Case>,
    size_x: i32,
    size_y: i32,
}

impl Map {
    pub fn new() -> Self {
        Map {
            name: "default".into(),
            list_items: Vec::new(),
            size_x: 0,
            size_y: 0,
        }
    }

    pub fn set_case(&mut self, x: &i32, y: &i32, type_case: &TypeCase) {
        let case: Option<&mut Case> = self.list_items
            .iter_mut()
            .filter(|case| case.get_x() == x && case.get_y() == y)
            .next();

        match case {
            Some(case_to_edit) => {
                case_to_edit.set_flag(type_case.clone());
            }
            None => (),
        }
    }

    pub fn draw(&self, window: &Window, edit_x: &i32, edit_y: &i32, mode: &MapWindowMode) {
        window.clear();
        window.draw_box('|', '-');

        match *mode {
            MapWindowMode::Normal => {
                self.list_items.iter().for_each(
                    |case| { case.draw(&window); },
                );
            }
            MapWindowMode::Edit => {
                self.list_items.iter().for_each(
                    |case| if case.get_x() == edit_x &&
                        case.get_y() == edit_y
                    {
                        window.attron(A_REVERSE);
                        case.draw(&window);
                        window.attroff(A_REVERSE);
                    } else {
                        case.draw(&window);
                    },
                );
            }
        }


        window.refresh();
    }

    pub fn create_empty(&mut self, size_x: i32, size_y: i32) {
        for x in 1..size_x {
            for y in 1..size_y {
                self.list_items.push(Case::new(x, y, TypeCase::Void));
            }
        }

        self.size_x = size_x;
        self.size_y = size_y;
    }

    pub fn add_case(&mut self, c: Case) {
        self.list_items.push(c);
    }

    pub fn get_size_x(&self) -> i32 {
        self.size_x
    }

    pub fn set_width_height(&mut self, x: i32, y: i32) {
        self.size_x = x;
        self.size_y = y;
    }

    pub fn get_size_y(&self) -> i32 {
        self.size_y
    }

    pub fn save(&self) -> Result<(), String> {
        let filename: String = format!("{}.map", &self.name);

        match File::create(Path::new(&filename)) {
            Ok(_) => (),
            Err(err) => return Err(format!("Could not create file : {:?}", err)),
        }

        let file = File::open(filename);

        match file {
            Ok(mut file) => {
                let flags: String = self.list_items.iter().fold("".into(), |mut acc, case| {
                    acc.push_str(case.get_flag_str());
                    acc
                });
                let to_write: String = format!(
                    "w:{}
                    h:{}
                    d:
                    {}",
                    self.size_x,
                    self.size_y,
                    flags
                );

                match file.write(to_write.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("Could not write to file: {:?}", err)),
                }
            }
            Err(err) => Err(format!("Could not open file: {:?}", err)),
        }
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}
