extern crate pathfinder_ncurses_rs;

use pathfinder_ncurses_rs::blocks::map::Map;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[test]
fn test_map_write() {
    let mut map: Map = Map::new();
    map.set_name("test_10x10_empty".into());
    map.create_empty(10, 10);

    let wanted: Result<(), String> = Result::Ok(());
    assert_eq!(map.save(), wanted);

    let mut file_genereted = File::open(Path::new("test_10x10_empty.map")).unwrap();
    let mut file_wanted = File::open(Path::new("./tests/10x10_empty.map")).unwrap();

    let mut file_generated_content: Vec<u8> = vec!();
    let mut file_wanted_content: Vec<u8> = vec!();
    file_genereted.read_to_end(&mut file_generated_content);
    file_wanted.read_to_end(&mut file_wanted_content);

    assert_eq!(file_wanted_content, file_generated_content);
}