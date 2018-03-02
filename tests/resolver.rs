extern crate pathfinder_ncurses_rs;

use pathfinder_ncurses_rs::blocks::map::Map;
use pathfinder_ncurses_rs::blocks::mapparser::{load_map, MapInfos};
use pathfinder_ncurses_rs::pathfinding::resolver::{Resolver, ResolverError};

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[test]
fn test_resolver_ok() {
    let result_load_map = load_map("./tests/10x10_resolver_ok.map".into());
    assert_eq!(result_load_map.is_ok(), true);

    let mut map_infos: MapInfos = result_load_map.unwrap();
    let mut map: Map = map_infos.create_map();

    let resolver = Resolver::try_new(&map.list_items);
    assert_eq!(resolver.is_ok(), true);
}

#[test]
fn test_resolver_miss_start() {
    let result_load_map = load_map("./tests/10x10_resolver_miss_start.map".into());
    assert_eq!(result_load_map.is_ok(), true);

    let mut map_infos: MapInfos = result_load_map.unwrap();
    let mut map: Map = map_infos.create_map();

    let resolver = Resolver::try_new(&map.list_items);
    assert_eq!(resolver, Err(ResolverError::NoStartPoint));
}

#[test]
fn test_resolver_miss_end() {
    let result_load_map = load_map("./tests/10x10_resolver_miss_end.map".into());
    assert_eq!(result_load_map.is_ok(), true);

    let mut map_infos: MapInfos = result_load_map.unwrap();
    let mut map: Map = map_infos.create_map();

    let resolver = Resolver::try_new(&map.list_items);
    assert_eq!(resolver, Err(ResolverError::NoEndPoint));
}

#[test]
fn test_resolver_miss_both() {
    let result_load_map = load_map("./tests/10x10_resolver_miss_both.map".into());
    assert_eq!(result_load_map.is_ok(), true);

    let mut map_infos: MapInfos = result_load_map.unwrap();
    let mut map: Map = map_infos.create_map();

    let resolver = Resolver::try_new(&map.list_items);
    assert_eq!(resolver, Err(ResolverError::NoStartNorEnd));
}