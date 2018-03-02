extern crate pathfinder_ncurses_rs;

use pathfinder_ncurses_rs::blocks::map::Map;
use pathfinder_ncurses_rs::blocks::mapparser::{load_map, MapInfos};
use pathfinder_ncurses_rs::path::node::Node;
use pathfinder_ncurses_rs::path::resolver::{Resolver, ResolverError};

#[test]
pub fn test_nodes_neighbours() {
    let top_left_node: Node = Node::new(0, 0);

    let neighbour_right: Node = Node::new(1, 0);
    let neighbour_up: Node = Node::new(0, 1);

    let totally_not_a_neighbour: Node = Node::new(10, 0);
    let not_a_neighbour: Node = Node::new(1, 1);

    assert_eq!(top_left_node.is_neighbor(&neighbour_right), true);
    assert_eq!(top_left_node.is_neighbor(&neighbour_up), true);
    assert_eq!(top_left_node.is_neighbor(&totally_not_a_neighbour), false);
    assert_eq!(top_left_node.is_neighbor(&not_a_neighbour), false);
}

#[test]
pub fn test_nodes_map() {
    let mut map: Map = Map::new();
    map.create_empty(10, 10);

    let nodes: Vec<Node> = map.list_items.iter().map(|a| a.create_node()).collect();
    assert_eq!(nodes.len(), 100);

    let node_middle: Node = Node::new(5, 5);
    let neighbours_middle = node_middle.get_neighbors(&nodes);

    let node_top_left: Node = Node::new(1, 1);
    let neighbours_top_left = node_top_left.get_neighbors(&nodes);

    let node_bot_right: Node = Node::new(10, 10);
    let neighbours_bot_right = node_bot_right.get_neighbors(&nodes);

    assert_eq!(neighbours_middle.len(), 4);
    assert_eq!(neighbours_top_left.len(), 2);
    assert_eq!(neighbours_bot_right.len(), 2);
}

#[test]
pub fn test_nodes_equality() {
    let node: Node = Node::new(0, 0);
    let other: Node = Node::new(0, 0);

    assert_eq!(node, other);
    assert_eq!(&node, &other);
}

#[test]
pub fn test_nodes_resolver() {
    let map_infos: MapInfos = load_map("./tests/10x10_resolver_ok.map".into()).unwrap();
    let map = map_infos.create_map();

    let resolver: Resolver = Resolver::try_new(&map.list_items).unwrap();
    let result = resolver.resolve();

    assert_eq!(result.is_ok(), true);
}