extern crate pathfinder_ncurses_rs;

use pathfinder_ncurses_rs::blocks::map::Map;

#[test]
fn test_map_write() {
    let mut map: Map = Map::new();
    map.set_name("test_10x10_empty".into());
    map.create_empty(10, 10);

    let wanted: Result<(), String> = Result::Ok(());
    assert_eq!(map.save(), wanted);
}