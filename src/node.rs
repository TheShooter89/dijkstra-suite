//! This modules defines types and traits for a graph node
//!
//! ## The`Node` type
//!
//! `Node` type plays a central role in this suite.
//!
//! Each `Node` has an id and a weight, both generic over types implementing `NodeId`
//! and `NodeWeight` traits respectively. Both have blanket implementations over generic types
//! so you can use any types you want as long as it implements the required traits
//!
//! ```rust
//! use dijkstra_suite::node::Node;
//!
//! let node_a: Node<(i32, i32), f64> = Node {
//!     weight: 1.0,
//!     ..Default::default()
//! };
//!
//! let node_b = Node {
//!     id: (0, 0),
//!     weight: 1.0,
//!     neighbours: vec![],
//! };
//!
//! let node_c: Node<(i32, i32), f32> = Node::default();
//!
//! assert_eq!(node_a, node_b);
//! assert_eq!(node_c.weight, 0.0)
//! ```
//!
//! ## The`NodeId` trait
//!
//! `Nodeid` is a super trait that ensures the id of the node is comparable and hashable
//! (current implementation uses an `HashMap` under the hood), requiring the implementing
//! type also to implement `Hash`, `Eq` and `Default`
//!
//!
//! ## The`NodeWeight` trait
//!
//! `NodeWeight` is a super trait that ensures the weight of the node is a number or number-like
//! types, requiring the implementing type also to implement `Add`, `Div`, `Mul`, `Rem`, `Sub`
//! and `PartialEq`

use std::{
    hash::Hash,
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Debug, Clone)]
pub struct Node<I: NodeId, W: NodeWeight> {
    pub id: I,
    pub weight: W,
    pub neighbours: Vec<NodeConnection<W, I>>,
}

impl<I: NodeId, W: NodeWeight> Default for Node<I, W> {
    fn default() -> Self {
        Node {
            id: I::default(),
            weight: W::default(),
            neighbours: vec![],
        }
    }
}

impl<I: NodeId, W: NodeWeight> PartialEq for Node<I, W> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.weight == other.weight && self.neighbours == other.neighbours
    }
}

pub trait NodeId: Default + Eq + Hash {}

impl<T> NodeId for T where T: Default + Eq + Hash {}

pub trait NodeWeight:
    Default
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

impl<T> NodeWeight for T where
    T: Default
        + PartialEq
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
{
}

#[derive(Debug, Clone)]
pub struct NodeConnection<W: NodeWeight, I: NodeId> {
    pub weight: W,
    pub to: I,
}

impl<W: NodeWeight, I: NodeId> Default for NodeConnection<W, I> {
    fn default() -> Self {
        NodeConnection {
            weight: W::default(),
            to: I::default(),
        }
    }
}

impl<W: NodeWeight, I: NodeId> AsRef<NodeConnection<W, I>> for NodeConnection<W, I> {
    fn as_ref(&self) -> &NodeConnection<W, I> {
        self
    }
}

impl<W: NodeWeight, I: NodeId> PartialEq for NodeConnection<W, I> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.to == other.to
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node() {
        let node_a = Node {
            weight: 1,
            ..Default::default()
        };

        let node_b = Node {
            id: (0, 0),
            weight: 1,
            neighbours: vec![],
        };

        let node_c: Node<(i32, i32), i32> = Node::default();

        assert_eq!(node_a, node_b);
        assert_eq!(node_c.weight, 0)
    }

    #[test]
    fn test_node_float() {
        let node_a: Node<(i32, i32), f64> = Node {
            weight: 1.0,
            ..Default::default()
        };

        let node_b = Node {
            id: (0, 0),
            weight: 1.0,
            neighbours: vec![],
        };

        let node_c: Node<(i32, i32), f32> = Node::default();

        assert_eq!(node_a, node_b);
        assert_eq!(node_c.weight, 0.0)
    }
}
