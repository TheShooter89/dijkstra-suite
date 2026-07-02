//! Strategy traits to handle multiple implementation logics
//!
//! ## The`strategy` module
//!
//! This module defines traits and types to implement a customized strategy pattern
//! used to handle the algorithm logic implementation
//!
//! ## `ImplementationStrategy` trait
//!
//! Implement the `ImplementationStrategy` trait to create a strategy that computes the algorithm
//!
//! The `StrategyRunner` is a convenience trait, that automatically implements a `strategy()`
//! method that runs an implementation, selected by specifying the strategy type on function type
//! spec when called
//!
//! ## Example usage
//!
//! ```rust
//! use dijkstra_suite::{
//!     graph::Graph,
//!     node::{NodeId, NodeWeight},
//!     path::Path,
//!     strategy::{ImplementationStrategy, StrategyRunner}
//! };
//!
//! #[derive(Debug)]
//! struct TestStrategy {}
//!
//! impl ImplementationStrategy for TestStrategy {
//!     type Opts = ();
//!
//!     fn run<I: NodeId, W: NodeWeight>(
//!         graph: &Graph<I, W>,
//!         from: I,
//!         to: I,
//!         options: Self::Opts,
//!     ) -> Result<Path<I, W>, String> {
//!         Ok(Path::default())
//!     }
//! }
//!
//! #[derive(Debug)]
//! struct TestRunner {}
//!
//! impl StrategyRunner for TestRunner {}
//!
//! let path = TestRunner::strategy::<TestStrategy, i32, i32>(&Graph::default(), 0, 99, ());
//!
//! assert_eq!(path, Ok(Path::default()))
//! ```

use crate::{
    graph::Graph,
    node::{NodeId, NodeWeight},
    path::Path,
};

pub trait ImplementationStrategy {
    type Opts: Default;

    fn run<I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
        options: Self::Opts,
    ) -> Result<Path<I, W>, String> {
        Err("Strategy not implemented".into())
    }
}

pub trait StrategyRunner {
    fn strategy<Strategy: ImplementationStrategy, I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
        options: Strategy::Opts,
    ) -> Result<Path<I, W>, String> {
        Strategy::run(&graph, from, to, options)
        // Ok(Path::default())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        graph::Graph,
        path::Path,
        strategy::{ImplementationStrategy, StrategyRunner},
    };

    #[test]
    fn test_strategy() {
        #[derive(Debug)]
        struct TestStrategy {}

        impl ImplementationStrategy for TestStrategy {
            type Opts = ();

            fn run<I: crate::node::NodeId, W: crate::node::NodeWeight>(
                graph: &crate::graph::Graph<I, W>,
                from: I,
                to: I,
                options: Self::Opts,
            ) -> Result<crate::path::Path<I, W>, String> {
                Ok(Path::default())
            }
        }

        #[derive(Debug)]
        struct TestRunner {}

        impl StrategyRunner for TestRunner {}

        let path = TestRunner::strategy::<TestStrategy, i32, i32>(&Graph::default(), 0, 99, ());

        assert_eq!(path, Ok(Path::default()))
    }
}
