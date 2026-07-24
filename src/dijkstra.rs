//! A Dijkstra's algorithm implementation that aims to be simple to use and fast to run
//!
//! This is the central pillar of `dijkstra_suite` crate.
//! `dijkstra` module implements the algorithm logic, exposes the main function to compute
//! the shortest path and provides the interface to the core parts of implementation
//!
//! ## Versioned implementations for multi-strategy support
//!
//! Main implementation should be `v1` and that should be used for "default exports" from
//! `dijkstra` module
//!
//! Versioned submoduling allows multiple implementations to live and be maintened together
//! and easily switched
//!
//! ## Usage
//!
//! Create a `Graph`, define the start and the end node ids, then call `dijkstra_path()` function.
//! Returned result is a sequence of node ids that represents the shortest path possible
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

use crate::{
    error::DijkstraError,
    graph::Graph,
    node::{NodeId, NodeWeight},
    path::Path,
    strategy::Strategy,
    v1::strategy::DijkstraAlgorithm,
    log_info
};

pub mod v1;

/// compute the best possible path in a graph using Dijkstra algorithm
///
/// it returns a `Path` holding both the weight and the steps of the path
pub fn dijkstra_path<I: NodeId, W: NodeWeight>(
    graph: &Graph<I, W>,
    from: I,
    to: I,
) -> Result<Path<I, W>, DijkstraError> {
    log_info!("dijkstra_path function");

    Strategy::execute::<DijkstraAlgorithm, I, W>(graph, from, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let graph: Graph<String, i32> = Graph::default();
        let start = "A".to_string();
        let end = "B".to_string();

        // let result = dijkstra_path(&graph, start, end);

        // assert_eq!(result, Err("blyat".into()));
        // assert_eq!(result, Ok(Path::default()));
    }
}
