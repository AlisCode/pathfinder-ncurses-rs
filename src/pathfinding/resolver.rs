use blocks::case::Case;
use pathfinding::node::Node;
use pathfinding::path::Path;

#[derive(Debug, PartialEq)]
pub struct Resolver {
    list_nodes: Vec<Node>,
    open_list: Vec<Node>,
    closed_list: Vec<Node>,
    curr_node: Node,
    end_node: Node,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolverError {
    NoStartPoint,
    NoEndPoint,
    NoStartNorEnd,
    NoPathFound,
}

impl Resolver {
    pub fn try_new(list_cases: &Vec<Case>) -> Result<Self, ResolverError> {
        let list_nodes: Vec<Node> = list_cases
            .iter()
            .filter_map(|a| match a.is_crossable() && !a.is_start_point() &&
                !a.is_end_point() {
                true => Some(a.create_node()),
                false => None,
            })
            .collect();

        let start_point: Option<Node> = list_cases
            .iter()
            .filter_map(|a| match a.is_start_point() {
                true => Some(a.create_node()),
                false => None,
            })
            .next();

        let end_point: Option<Node> = list_cases
            .iter()
            .filter_map(|a| match a.is_end_point() {
                true => Some(a.create_node()),
                false => None,
            })
            .next();

        match (start_point, end_point) {
            (Some(start), Some(end)) => Ok(Resolver {
                list_nodes,
                open_list: Vec::new(),
                closed_list: Vec::new(),
                curr_node: start,
                end_node: end,
            }),
            (None, None) => Err(ResolverError::NoStartNorEnd),
            (Some(start), None) => Err(ResolverError::NoEndPoint),
            (None, Some(end)) => Err(ResolverError::NoStartPoint),
        }
    }

    pub fn resolve(&mut self) -> Result<Path, ResolverError> {
        while self.open_list.len() > 0 {
            if self.curr_node.has_same_coordinates(&self.end_node) {
                // Return the path from the end node
            }

            // Sorts the open_list by fcost
            self.open_list.sort_by(|a, b| a.sort_by_fcost(b));

            if let Some(first) = self.open_list.pop() {
                self.curr_node = first;
            }
        }

        Err(ResolverError::NoPathFound)
    }
}