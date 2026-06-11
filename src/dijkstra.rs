//! A Dijkstra's algorithm implementation that aims to be simple to use and fast to run
//!
//! This is the central pillar of `dijkstra_suite` crate.
//! `dijkstra` module implements the algorithm logic, exposes the main function to compute
//! the shortest path and provides the interface to the core parts of implementation
//!
//! ## Usage
//!
//! Create a `Graph`, define the start and the end node ids, then call `dijkstra_path()` function.
//! Returned result is a sequence of node ids that represents the shortest path possible
//!
//! ```rust
//! use dijkstra_suite::dijkstra::dijkstra_path;
//! use dijkstra_suite::graph::Graph;
//!
//! let graph: Graph<String, i32> = Graph::default();
//! let start = "A".to_string();
//! let end = "B".to_string();
//!
//! let result = dijkstra_path(&graph, &start, &end);
//!
//! assert_eq!(result, Ok((0, vec![])));
//! ```

use crate::{
    graph::Graph,
    node::{NodeId, NodeWeight},
};

/// compute the best possible path in a graph using Dijkstra algorithm
///
/// it returns a tuple, with total weight of the path and an ordered sequence of node ids
/// in the form `(total_weight: NodeWeight, steps: Vec<NodeId>)`
pub fn dijkstra_path<I: NodeId, W: NodeWeight>(
    graph: &Graph<I, W>,
    start: &I,
    end: &I,
) -> Result<(W, Vec<I>), String> {
    println!("dijkstra_path function");

    Ok((W::default(), vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let graph: Graph<String, i32> = Graph::default();
        let start = "A".to_string();
        let end = "B".to_string();

        let result = dijkstra_path(&graph, &start, &end);

        // assert_eq!(result, Err("blyat".into()));
        assert_eq!(result, Ok((0, vec![])));
    }
}
