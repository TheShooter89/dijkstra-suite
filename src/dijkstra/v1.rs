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
//!
//! let graph: Graph<String, i32> = Graph::default();
//! let start = "A".to_string();
//! let end = "B".to_string();
//!
//! let result = dijkstra_path(&graph, start, end);
//!
//! assert_eq!(result, Ok(Path::default()));
//! ```

pub mod strategy;
