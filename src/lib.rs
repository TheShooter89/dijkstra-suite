//! A Dijkstra's algorithm implementation that aims to be simple to use and fast to run
//!
//! *simple.* nodes id and its cost are defined by yuor own types
//!
//! *fast.* ok, it's still a work-in-progress, but the goal is a fast computing with as
//! less allocations as possible
//!
//! ```toml
//! [dependencies]
//! dijkstra-suite = "0.1.0-beta.1"
//! ```
//!
//! ## Usage
//!
//! Create a `Graph`, define the start and the end node ids, then call `dijkstra_path()` function.
//! Returned result is a path of `Path` type, represented as an ordinated sequence of node ids
//! along with the total weight of the path (the sum of all node weights of the path)
//!
//! ```rust
//! use dijkstra_suite::dijkstra::dijkstra_path;
//! use dijkstra_suite::graph::Graph;
//! use dijkstra_suite::path::Path;
//! use dijkstra_suite::node::{Node, NodeConnection};
//!
//! let mut graph: Graph<&str, f32> = Graph::default();
//! let node_a = Node {
//!     id: "A",
//!     weight: 0.0,
//!     neighbours: vec![
//!         NodeConnection {
//!             from: "A",
//!             to: "B",
//!             weight: 7.0,
//!         },
//!         NodeConnection {
//!             from: "A",
//!             to: "E",
//!             weight: 1.0,
//!         },
//!     ],
//! };
//!
//! let node_b = Node {
//!     id: "B",
//!     weight: 0.0,
//!     neighbours: vec![
//!         NodeConnection {
//!             from: "B",
//!             to: "A",
//!             weight: 7.0,
//!         },
//!         NodeConnection {
//!             from: "B",
//!             to: "C",
//!             weight: 3.0,
//!         },
//!         NodeConnection {
//!             from: "B",
//!             to: "E",
//!             weight: 8.0,
//!         },
//!     ],
//! };
//!
//! let node_c = Node {
//!     id: "C",
//!     weight: 0.0,
//!     neighbours: vec![
//!         NodeConnection {
//!             from: "C",
//!             to: "B",
//!             weight: 3.0,
//!         },
//!         NodeConnection {
//!             from: "C",
//!             to: "D",
//!             weight: 6.0,
//!         },
//!         NodeConnection {
//!             from: "C",
//!             to: "E",
//!             weight: 2.0,
//!         },
//!     ],
//! };
//!
//! let node_d = Node {
//!     id: "D",
//!     weight: 0.0,
//!     neighbours: vec![
//!         NodeConnection {
//!             from: "D",
//!             to: "C",
//!             weight: 6.0,
//!         },
//!         NodeConnection {
//!             from: "D",
//!             to: "E",
//!             weight: 7.0,
//!         },
//!     ],
//! };
//!
//! let node_e = Node {
//!     id: "E",
//!     weight: 0.0,
//!     neighbours: vec![
//!         NodeConnection {
//!             from: "E",
//!             to: "A",
//!             weight: 1.0,
//!         },
//!         NodeConnection {
//!             from: "E",
//!             to: "B",
//!             weight: 8.0,
//!         },
//!         NodeConnection {
//!             from: "E",
//!             to: "C",
//!             weight: 2.0,
//!         },
//!         NodeConnection {
//!             from: "E",
//!             to: "D",
//!             weight: 7.0,
//!         },
//!     ],
//! };
//!
//! graph.insert(node_a.id, node_a);
//! graph.insert(node_b.id, node_b);
//! graph.insert(node_c.id, node_c);
//! graph.insert(node_d.id, node_d);
//! graph.insert(node_e.id, node_e);
//!
//! let result = dijkstra_path(&graph, "B", "D");
//!
//! let expected_path: Path<&str, f32> = Path {
//!     weight: 9.0,
//!     steps: vec!["B", "C", "D"],
//! };
//!
//! assert_eq!(result.unwrap(), expected_path)
//!
//! ```

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/nanoid"
)]

pub mod compute;
pub mod dijkstra;
pub mod graph;
pub mod node;
pub mod path;
pub mod strategy;

pub use dijkstra::*;

#[cfg(test)]
mod tests {
    use crate::{graph::Graph, node::Node, path::Path};

    use super::*;

    #[test]
    fn it_works() {
        let mut graph: Graph<String, i32> = Graph::default();
        let start = "A".to_string();
        let end = "B".to_string();

        graph.insert(start.clone(), Node::default());
        graph.insert(end.clone(), Node::default());

        // let result = dijkstra_path(&graph, start, end);

        // assert_eq!(result, Err("blyat".into()));
        // assert_eq!(result, Ok(Path::default()));
    }
}
