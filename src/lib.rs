//! A Dijkstra's algorithm implementation that aims to be simple to use and fast to run
//!
//! ** simple.** nodes id and its cost are defined by yuor own types
//!
//! ```toml
//! [dependencies]
//! dijkstra-suite = "0.1.0-alpha.0"
//! ```
//!
//! ## Usage
//!
//! Just import the `add()` function and call it
//!
//! ```rust
//! use dijkstra_suite::dijkstra_path;
//! use dijkstra_suite::graph::Graph;
//!
//! let graph: Graph<String, i32> = Graph::default();
//! let start = "A".to_string();
//! let end = "B".to_string();
//!
//! let result = dijkstra_path(&graph, &start, &end);
//!
//! assert_eq!(result, Ok(()));
//! ```

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/nanoid"
)]

mod dijkstra;
pub mod graph;
pub mod node;

pub use dijkstra::*;

#[cfg(test)]
mod tests {
    use crate::graph::Graph;

    use super::*;

    #[test]
    fn it_works() {
        let graph: Graph<String, i32> = Graph::default();
        let start = "A".to_string();
        let end = "B".to_string();

        let result = dijkstra_path(&graph, &start, &end);

        // assert_eq!(result, Err("blyat".into()));
        assert_eq!(result, Ok(()));
    }
}
