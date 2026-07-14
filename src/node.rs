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
    fmt::Debug,
    hash::Hash,
    ops::{Add, Deref, Div, Mul, Rem, Sub},
};

#[derive(Debug, Clone)]
pub struct Node<I: NodeId, W: NodeWeight> {
    pub id: I,
    pub weight: W,
    pub neighbours: Vec<NodeConnection<I, W>>,
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

pub trait NodeId: Debug + Default + Clone + Eq + PartialOrd + Hash {}

impl<T> NodeId for T where T: Debug + Default + Clone + Eq + PartialOrd + Hash {}

pub trait NodeWeight:
    Debug
    + Default
    + Clone
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

impl<T> NodeWeight for T where
    T: Debug
        + Default
        + Clone
        + PartialEq
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
{
}

// #[derive(Debug, Default, Clone, PartialEq)]
// pub struct Weight<T: NodeWeight>(pub T);
//
// impl<T: NodeWeight> Eq for Weight<T> {}
//
// impl<T: NodeWeight> Ord for Weight<T> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.0
//             .partial_cmp(&other.0)
//             .expect("Weight contains a value with no defined order (e.g. NaN)")
//     }
// }
//
// impl<T: NodeWeight> PartialOrd for Weight<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl<T> Deref for Weight<T>
// where
//     T: Debug
//         + Default
//         + Clone
//         + PartialEq
//         + PartialOrd
//         + Ord
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Rem<Output = T>,
// {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<T: NodeWeight> Add for Weight<T> {
//     type Output = Weight<T>;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Weight(self.0 + rhs.0)
//     }
// }
//
// impl<T: NodeWeight> Sub for Weight<T> {
//     type Output = Weight<T>;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Weight(self.0 - rhs.0)
//     }
// }
//
// impl<T: NodeWeight> Mul for Weight<T> {
//     type Output = Weight<T>;
//
//     fn mul(self, rhs: Self) -> Self::Output {
//         Weight(self.0 * rhs.0)
//     }
// }
//
// impl<T: NodeWeight> Div for Weight<T> {
//     type Output = Weight<T>;
//
//     fn div(self, rhs: Self) -> Self::Output {
//         Weight(self.0 / rhs.0)
//     }
// }
//
// impl<T: NodeWeight> Rem for Weight<T> {
//     type Output = Weight<T>;
//
//     fn rem(self, rhs: Self) -> Self::Output {
//         Weight(self.0 % rhs.0)
//     }
// }

#[derive(Debug, Clone)]
pub struct NodeConnection<I: NodeId, W: NodeWeight> {
    pub from: I,
    pub to: I,
    pub weight: W,
}

impl<I: NodeId, W: NodeWeight> Default for NodeConnection<I, W> {
    fn default() -> Self {
        NodeConnection {
            from: I::default(),
            to: I::default(),
            weight: W::default(),
        }
    }
}

impl<I: NodeId, W: NodeWeight> AsRef<NodeConnection<I, W>> for NodeConnection<I, W> {
    fn as_ref(&self) -> &NodeConnection<I, W> {
        self
    }
}

impl<I: NodeId, W: NodeWeight> PartialEq for NodeConnection<I, W> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.weight == other.weight
    }
}

impl<I: NodeId, W: NodeWeight> Eq for NodeConnection<I, W> {}

impl<I: NodeId, W: NodeWeight> Ord for NodeConnection<I, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.partial_cmp(&other.weight).expect("bla")
    }
}

impl<I: NodeId, W: NodeWeight> PartialOrd for NodeConnection<I, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
