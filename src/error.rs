//! customized library-wide error handling
//!
//! ## The`DijkstraError` type
//!
//! The `DijkstraError` enum is a centralized error type, every library
//! component which may return an error uses [`DijkstraError`]
//!
//! This type implements the [`Error`] trait manually, so to avoid introducing
//! a dependency
//!
//! ```rust
//! use dijkstra_suite::error::DijkstraError;
//! use dijkstra_suite::compute::DistanceFromSource;
//!
//! let mut distances = DistanceFromSource::default();
//!
//! distances.set_distance((0, 0), 0, None);
//! let node_error = DijkstraError::ComputePathError("end node not found".into());
//!
//! let result = distances.compute_path((0, 2));
//!
//! assert_eq!(result, Err(node_error));
//! ```

use std::{error::Error, fmt::Display};

/// centralized error type
///
/// ## The `DijkstraError` error type
///
/// This enum is a centralized error type used throughout the library
/// It implements the [`Error`] trait manually so to avoid an external
/// dependency
///
/// ## Example
///
/// Main function [`dijkstra_path()`](crate::dijkstra::dijkstra_path)
/// as well as internals like [`DistanceFromSource::compute_path()`](crate::compute::DistanceFromSource::compute_path)
/// uses `DijkstraError`:
///
/// ```rust
/// use dijkstra_suite::error::DijkstraError;
/// use dijkstra_suite::compute::DistanceFromSource;
///
/// let mut distances = DistanceFromSource::default();
///
/// distances.set_distance((0, 0), 0, None);
/// let node_error = DijkstraError::ComputePathError("end node not found".into());
///
/// let result = distances.compute_path((0, 2));
///
/// assert_eq!(result, Err(node_error));
/// ```
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum DijkstraError {
    /// error while generating the generic zero
    GenericZeroError(String),
    DistanceEntryNotFound(String),
    ComputePathError(String),
    GraphNodeNotFound(String),
    StrategyError(String),
}

impl Display for DijkstraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DijkstraError::GenericZeroError(e) => {
                write!(f, "error retrieving generic zero: {}", e)
            }
            DijkstraError::DistanceEntryNotFound(e) => {
                write!(f, "entry not found in current distances list. error: {}", e)
            }
            DijkstraError::ComputePathError(e) => {
                write!(f, "an error occurred computing the path: {}", e)
            }
            DijkstraError::GraphNodeNotFound(e) => {
                write!(f, "node not found in graph: {}", e)
            }
            DijkstraError::StrategyError(e) => {
                write!(f, "StrategyError: {}", e)
            }
        }
    }
}

impl Error for DijkstraError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            // this forwards an eventual Error type property of variants, for eaxmple
            // if variant would be DijkstraError::ComputePathError(std::io::Error)
            // DijkstraError::ComputePathError(e) => Some(e),
            _ => None,
        }
    }
}
