use std::{error::Error, fmt::Display};

#[non_exhaustive]
#[derive(Debug)]
pub enum DijkstraError {
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
