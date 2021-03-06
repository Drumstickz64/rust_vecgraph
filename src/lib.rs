use std::fmt::{self, Display, Formatter};

pub type EdgeData = Vec<usize>;

#[derive(Debug, PartialEq)]
pub struct EdgeAdditionError(pub String);

#[derive(Debug, PartialEq)]
pub struct EdgeGetError(pub String);

#[derive(PartialEq, Clone)]
pub struct Graph<T> {
    nodes: Vec<T>,
    edges: Vec<EdgeData>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        self.edges.push(EdgeData::new());
        index
    }

    pub fn get_node(&self, node_idx: usize) -> Option<&T> {
        self.nodes.get(node_idx)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &T> {
        self.nodes.iter()
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), EdgeAdditionError> {
        if to >= self.nodes.len() {
            return Err(EdgeAdditionError(format!(
                "parameter to out of range, to: {}, length: {}",
                to,
                self.edges.len()
            )));
        }

        if from >= self.nodes.len() {
            return Err(EdgeAdditionError(format!(
                "parameter from out of range, from: {}, length: {}",
                from,
                self.edges.len()
            )));
        }

        if from == to {
            return Err(EdgeAdditionError("Node cannot point to itself".to_string()));
        }

        self.edges[from].push(to);
        Ok(())
    }

    pub fn get_edges_from(&self, idx: usize) -> Result<EdgeData, EdgeGetError> {
        if idx >= self.nodes.len() {
            return Err(EdgeGetError(format!(
                "index is out of bounds: index: {}, length: {}",
                idx,
                self.edges.len()
            )));
        }

        Ok(self.edges[idx].clone())
    }

    pub fn get_edges_to(&self, idx: usize) -> Result<EdgeData, EdgeGetError> {
        if idx >= self.nodes.len() {
            return Err(EdgeGetError(format!(
                "index is out of bounds: index: {}, length: {}",
                idx,
                self.edges.len()
            )));
        }

        let mut edges = Vec::new();
        for (edge_idx, edge_data) in self.edges.iter().enumerate() {
            for &edge in edge_data.iter() {
                if edge == idx {
                    edges.push(edge_idx)
                }
            }
        }
        Ok(edges)
    }

    pub fn get_edges(&self, idx: usize) -> Result<EdgeData, EdgeGetError> {
        if idx >= self.nodes.len() {
            return Err(EdgeGetError(format!(
                "index is out of bounds: index: {}, length: {}",
                idx,
                self.edges.len()
            )));
        }

        let mut result = Vec::new();
        result.append(&mut self.get_edges_from(idx)?);
        result.append(&mut self.get_edges_to(idx)?);
        Ok(result)
    }

    pub fn remove_node(&mut self, idx: usize) -> T {
        if idx >= self.nodes.len() {
            panic!(
                "index index out of range: index is {}, but len is {}",
                idx,
                self.nodes.len(),
            );
        }

        self.edges.remove(idx);
        for edge_data in self.edges.iter_mut() {
            *edge_data = edge_data
                .iter()
                .filter_map(|&edge| if edge != idx { Some(edge) } else { None })
                .map(|edge| if edge > idx { edge - 1 } else { edge })
                .collect();
        }
        self.nodes.remove(idx)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.nodes.is_empty() {
            return None;
        }

        Some(self.remove_node(self.nodes.len() - 1))
    }
}

impl<T: Display> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (node, targets) in self.nodes.iter().zip(self.edges.iter()) {
            for target in targets.iter() {
                writeln!(f, "{} -> {}", node, self.nodes[*target])?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    fn get_test_graph_without_edges() -> Graph<i32> {
        let mut graph = Graph::new();

        graph.add_node(5);
        graph.add_node(1);
        graph.add_node(12);
        graph.add_node(100);

        graph
    }

    fn get_test_graph_with_edges() -> Graph<i32> {
        let mut graph = get_test_graph_without_edges();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 1).unwrap();
        graph.add_edge(0, 3).unwrap();

        graph
    }

    #[test]
    fn edge_connection() {
        let mut graph = get_test_graph_without_edges();
        assert!(graph.add_edge(0, 1).is_ok());
        assert!(graph.add_edge(0, 2).is_ok());
        assert!(graph.add_edge(2, 3).is_ok());
        assert!(graph.add_edge(12, 0).is_err());
    }

    #[test]
    fn print_graph() {
        let graph = get_test_graph_with_edges();

        let graph_str = "5 -> 100\n1 -> 12\n12 -> 1\n";
        assert_eq!(graph.to_string(), graph_str);
    }

    #[test]
    fn node_removal() {
        let mut graph = get_test_graph_with_edges();

        assert_eq!(graph.remove_node(1), 1);
        assert_eq!(graph.remove_node(1), 12);
        assert!(std::panic::catch_unwind(|| graph.clone().remove_node(6)).is_err());
        assert_eq!(graph.to_string(), "5 -> 100\n");
        assert_eq!(graph.pop(), Some(100));
        assert_eq!(graph.nodes().collect::<Vec<_>>(), vec![&5]);
        graph.pop();
        assert_eq!(graph.pop(), None);
    }

    #[test]
    fn getting_edges() {
        let graph_with_edges = get_test_graph_with_edges();
        let graph_without_edges = get_test_graph_without_edges();

        assert_eq!(graph_with_edges.get_edges_from(1), Ok(vec![2]));
        assert_eq!(graph_with_edges.get_edges_to(1), Ok(vec![2]));
        assert_eq!(graph_with_edges.get_edges(1), Ok(vec![2, 2]));
        assert_eq!(graph_with_edges.get_edges_from(3), Ok(vec![]));

        assert!(graph_without_edges.get_edges_from(5).is_err());
        assert!(graph_without_edges.get_edges_to(5).is_err());
        assert!(graph_without_edges.get_edges(5).is_err());
    }
}
