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

use crate::node::{Node, NodeId, NodeWeight};

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
    pub fn shortest_path(self) -> Vec<I> {
        //

        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph() {
        // let mut graph: Graph<i32, i32> = Graph(HashMap::new());
        let mut graph: Graph<i32, i32> = Graph::default();

        let node_a: Node<i32, i32> = Node::default();

        graph.insert(0, node_a);

        let first_node = graph.get(&0).unwrap();

        assert_eq!(first_node.id, 0)
    }
}
