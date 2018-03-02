use blocks::case::Case;
use path::node::Node;
use pathfinding::astar;

#[derive(Debug, PartialEq)]
pub struct Resolver {
    list_nodes: Vec<Node>,
    start_node: Node,
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
            .filter_map(|a| match a.is_crossable() && !a.is_start_point() {
                true => Some(a.create_node()),
                false => None,
            })
            .collect();

        let start_node: Option<Node> = list_cases
            .iter()
            .filter_map(|a| match a.is_start_point() {
                true => Some(a.create_node()),
                false => None,
            })
            .next();

        let end_node: Option<Node> = list_cases
            .iter()
            .filter_map(|a| match a.is_end_point() {
                true => Some(a.create_node()),
                false => None,
            })
            .next();

        match (start_node, end_node) {
            (Some(start), Some(end)) => Ok(Resolver {
                list_nodes,
                start_node: start,
                end_node: end,
            }),
            (None, None) => Err(ResolverError::NoStartNorEnd),
            (Some(_), None) => Err(ResolverError::NoEndPoint),
            (None, Some(_)) => Err(ResolverError::NoStartPoint),
        }
    }

    pub fn resolve(&self) -> Result<Vec<Node>, ResolverError> {
        let result = astar(&self.start_node, |n| n.get_neighbors(&self.list_nodes), |n| n.get_distance(&self.end_node), |n| n == &self.end_node);

        match result {
            Some((mut path, _)) => {
                path.remove(0);
                let length: usize = path.len();
                path.remove(length - 1);
                Ok(path)
            }
            None => Err(ResolverError::NoPathFound),
        }
    }
}