//! This modules defines types and traits for a graph
//!
//! ## The`Graph` type
//!
//! The `Graph` type is the other cornerstone of the suite, in pair with `Node` type
//!
//! This type implements the concrete graph representation upon which the dijkstra
//! implementeation will run to compute the shortest path
//!
//! ```rust
//! use std::collections::HashMap;
//! use dijkstra_suite::node::Node;
//! use dijkstra_suite::graph::Graph;
//!
//! let mut graph: Graph<i32, i32> = Graph(HashMap::new());
//!
//! let node_a: Node<i32, i32> = Node::default();
//!
//! graph.insert(0, node_a);
//!
//! let first_node = graph.get(&0).unwrap();
//!
//! assert_eq!(first_node.id, 0)
//! ```

use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    error::DijkstraError,
    node::{Node, NodeId, NodeWeight},
    path::Path,
    strategy::ImplementationStrategy,
    v1::strategy::DijkstraAlgorithm,
};

#[derive(Debug)]
pub struct Graph<I: NodeId, W: NodeWeight>(pub HashMap<I, Node<I, W>>);

impl<I: NodeId, W: NodeWeight> Default for Graph<I, W> {
    fn default() -> Self {
        Graph(HashMap::new())
    }
}

impl<I: NodeId, W: NodeWeight> Deref for Graph<I, W> {
    type Target = HashMap<I, Node<I, W>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: NodeId, W: NodeWeight> DerefMut for Graph<I, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I: NodeId, W: NodeWeight> PartialEq for Graph<I, W> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<I: NodeId, W: NodeWeight> Graph<I, W> {
    pub fn shortest_path(self, from: I, to: I) -> Result<Path<I, W>, DijkstraError> {
        self.shortest_path_with_opts(
            from,
            to,
            // since DijkstraAlgorithm strategy Opts type is the unit type ()
            // clippy wants to pass it explicitly
            // if DijkstraAlgorithm options type will vary, below should be used:
            // <DijkstraAlgorithm as ImplementationStrategy>::Opts::default(),
            (),
        )
        // SelectedStrategy::run(&self, from, to, SelectedStrategy::Opts::default())
    }
}

impl<I: NodeId, W: NodeWeight> Graph<I, W> {
    pub fn shortest_path_with_opts(
        self,
        from: I,
        to: I,
        options: <DijkstraAlgorithm as ImplementationStrategy>::Opts,
    ) -> Result<Path<I, W>, DijkstraError> {
        self.compute_shortest_path_with_opts::<DijkstraAlgorithm>(from, to, options)
        // SelectedStrategy::run(&self, from, to, SelectedStrategy::Opts::default())
    }
}

impl<I: NodeId, W: NodeWeight> Graph<I, W> {
    pub fn compute_shortest_path<SelectedStrategy: ImplementationStrategy>(
        self,
        from: I,
        to: I,
    ) -> Result<Path<I, W>, DijkstraError> {
        self.compute_shortest_path_with_opts::<SelectedStrategy>(
            from,
            to,
            SelectedStrategy::Opts::default(),
        )
    }
}

impl<I: NodeId, W: NodeWeight> Graph<I, W> {
    pub fn compute_shortest_path_with_opts<SelectedStrategy: ImplementationStrategy>(
        self,
        from: I,
        to: I,
        options: SelectedStrategy::Opts,
    ) -> Result<Path<I, W>, DijkstraError> {
        SelectedStrategy::run(&self, from, to, options)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        graph::Graph,
        node::{Node, NodeConnection},
        path::Path,
        strategy::Strategy,
        v1::strategy::DijkstraAlgorithm,
    };

    #[test]
    fn test_graph() {
        // let mut graph: Graph<i32, i32> = Graph(HashMap::new());
        let mut graph: Graph<i32, i32> = Graph::default();

        let node_a: Node<i32, i32> = Node::default();

        graph.insert(0, node_a);

        let first_node = graph.get(&0).unwrap();

        assert_eq!(first_node.id, 0)
    }

    #[test]
    fn test_graph_compute_shortest_path() {
        let mut graph: Graph<&str, f32> = Graph::default();
        let node_a = Node {
            id: "A",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "A",
                    to: "B",
                    weight: 7.0,
                },
                NodeConnection {
                    from: "A",
                    to: "E",
                    weight: 1.0,
                },
            ],
        };

        let node_b = Node {
            id: "B",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "B",
                    to: "A",
                    weight: 7.0,
                },
                NodeConnection {
                    from: "B",
                    to: "C",
                    weight: 3.0,
                },
                NodeConnection {
                    from: "B",
                    to: "E",
                    weight: 8.0,
                },
            ],
        };

        let node_c = Node {
            id: "C",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "C",
                    to: "B",
                    weight: 3.0,
                },
                NodeConnection {
                    from: "C",
                    to: "D",
                    weight: 6.0,
                },
                NodeConnection {
                    from: "C",
                    to: "E",
                    weight: 2.0,
                },
            ],
        };

        let node_d = Node {
            id: "D",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "D",
                    to: "C",
                    weight: 6.0,
                },
                NodeConnection {
                    from: "D",
                    to: "E",
                    weight: 7.0,
                },
            ],
        };

        let node_e = Node {
            id: "E",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "E",
                    to: "A",
                    weight: 1.0,
                },
                NodeConnection {
                    from: "E",
                    to: "B",
                    weight: 8.0,
                },
                NodeConnection {
                    from: "E",
                    to: "C",
                    weight: 2.0,
                },
                NodeConnection {
                    from: "E",
                    to: "D",
                    weight: 7.0,
                },
            ],
        };

        graph.insert(node_a.id, node_a);
        graph.insert(node_b.id, node_b);
        graph.insert(node_c.id, node_c);
        graph.insert(node_d.id, node_d);
        graph.insert(node_e.id, node_e);

        let result = graph.shortest_path("B", "D");

        let expected_path: Path<&str, f32> = Path {
            weight: 9.0,
            steps: vec!["B", "C", "D"],
        };

        // assert_eq!(1, 2);
        assert_eq!(result, Ok(expected_path))
    }
}
