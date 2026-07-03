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
//! ## `Strategy` type
//!
//! The `Strategy` type provides static methods to run a strategy, with default or customized options
//!
//! The `StrategyRunner` is a convenience trait, that automatically implements a `strategy()`
//! method that runs `Strategy::execute_with_opts()` under the hood
//!
//! ## Example usage
//!
//! ```rust
//! use dijkstra_suite::{
//!     graph::Graph,
//!     node::{NodeId, NodeWeight},
//!     path::Path,
//!     strategy::{ImplementationStrategy, Strategy}
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
//! let path = Strategy::execute_with_opts::<TestStrategy, i32, i32>(&Graph::default(), 0, 99, ());
//!
//! assert_eq!(path, Ok(Path::default()))
//! ```

use std::fmt::Debug;

use crate::{
    graph::Graph,
    node::{NodeId, NodeWeight},
    path::Path,
};

pub trait ImplementationStrategy {
    type Opts: Debug + Default;

    fn run<I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
        options: Self::Opts,
    ) -> Result<Path<I, W>, String> {
        Err(format!(
            "Strategy not implemented\n[
  graph: {:?}
  from: {:?}
  to: {:?}
  opts: {:?}
]",
            graph, from, to, options
        ))
        // Err("Strategy not implemented".into())
    }
}

pub trait StrategyRunner {
    fn strategy<SelectedStrategy: ImplementationStrategy, I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
        options: SelectedStrategy::Opts,
    ) -> Result<Path<I, W>, String> {
        Strategy::execute_with_opts::<SelectedStrategy, I, W>(graph, from, to, options)
    }
}

/// execute implementation strategies
///
/// ## `Strategy` type
///
/// The `Strategy` type provides static methods to run a strategy, with default or customized options
///
/// ### `execute()` method
///
/// This method runs a specific implementation strategy without specifiy any options,
/// the method automatically calls `SelectedStrategy::Opts::default()` under the hood
///
/// ### `execute_with_opts()` method
///
/// This method runs exactly as `execute()` with the only difference that you must also
/// specify the options for the choosen implementation strategy. Full control.
///
/// The `StrategyRunner` is a convenience trait, that automatically implements a `strategy()`
/// method that runs `Strategy::execute_with_opts()` under the hood
///
/// ## Example usage
///
/// ```rust
/// use dijkstra_suite::{
///     graph::Graph,
///     node::{NodeId, NodeWeight},
///     path::Path,
///     strategy::{ImplementationStrategy, Strategy}
/// };
///
/// #[derive(Debug)]
/// struct TestStrategy {}
///
/// impl ImplementationStrategy for TestStrategy {
///     type Opts = ();
///
///     fn run<I: NodeId, W: NodeWeight>(
///         graph: &Graph<I, W>,
///         from: I,
///         to: I,
///         options: Self::Opts,
///     ) -> Result<Path<I, W>, String> {
///         Ok(Path::default())
///     }
/// }
///
/// let path = Strategy::execute::<TestStrategy, i32, i32>(&Graph::default(), 0, 99);
///
/// assert_eq!(path, Ok(Path::default()));
///
/// let path = Strategy::execute_with_opts::<TestStrategy, i32, i32>(&Graph::default(), 0, 99, ());
///
/// assert_eq!(path, Ok(Path::default()))
/// ```
#[derive(Debug)]
pub struct Strategy {}

impl Strategy {
    pub fn execute<SelectedStrategy: ImplementationStrategy, I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
    ) -> Result<Path<I, W>, String> {
        SelectedStrategy::run(graph, from, to, SelectedStrategy::Opts::default())
        // Ok(())
    }

    pub fn execute_with_opts<SelectedStrategy: ImplementationStrategy, I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
        options: SelectedStrategy::Opts,
    ) -> Result<Path<I, W>, String> {
        SelectedStrategy::run(graph, from, to, options)
        // Ok(())
    }
}

impl StrategyRunner for Strategy {}

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

            // comment out below function to make the test fail
            // and print the error message
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

        if let Err(err_msg) = &path {
            println!("{}", err_msg.clone());
        }

        assert_eq!(path, Ok(Path::default()))
    }
}
