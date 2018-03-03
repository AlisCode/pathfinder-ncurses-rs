extern crate pathfinder_ncurses_rs;

use pathfinder_ncurses_rs::blocks::map::Map;
use pathfinder_ncurses_rs::blocks::mapparser::{load_map, MapInfos};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[test]
fn test_map_write() {
    let mut map: Map = Map::new();
    map.set_name("test_10x10_empty".into());
    map.create_empty(10, 10);

    let wanted: Result<(), String> = Result::Ok(());
    assert_eq!(map.save(), wanted);

    let mut file_genereted = File::open(Path::new("test_10x10_empty.map")).unwrap();
    let mut file_wanted = File::open(Path::new("./tests/10x10_empty.map")).unwrap();

    let mut file_generated_content: Vec<u8> = vec![];
    let mut file_wanted_content: Vec<u8> = vec![];
    file_genereted.read_to_end(&mut file_generated_content).unwrap();
    file_wanted.read_to_end(&mut file_wanted_content).unwrap();

    assert_eq!(file_wanted_content, file_generated_content);
}

#[test]
fn test_map_read() {
    let mut wanted_map: Map = Map::new();
    wanted_map.set_name("10x10_empty".into());
    wanted_map.create_empty(10, 10);

    let result_load_map = load_map("./tests/10x10_empty.map".into());
    assert_eq!(result_load_map.is_ok(), true);

    let map_infos: MapInfos = result_load_map.unwrap();
    let map = map_infos.create_map();

    map.list_items
        .into_iter()
        .zip(wanted_map.list_items.into_iter())
        .for_each(|(a, b)| {
            assert_eq!(a, b);
        });
}
