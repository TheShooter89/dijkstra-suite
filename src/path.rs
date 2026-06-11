//! Standardized path types and behaviours
//!
//! ## The`Path` type
//!
//! The `Path` type is the standardized way to represent a path across `dijkstra_suite`
//! and the rest of the world
//!
//! A `Path` is represented by two properties, `weight` and `steps`
//!
//! The `weight` props holds the total weight (or, cost) of the path
//!
//! The `steps` prop holds an ordered list of node ids that represent the actual path
//!
//! ```rust
//! use dijkstra_suite::path::Path;
//!
//! let mut path: Path<(i32, i32), i32> = Path::default();
//! println!("path: {:?}", path);
//!
//! path.steps.push((4, 2));
//! println!("path.weight: {:?}", path.weight);
//! println!("path.steps: {:?}", path.steps);
//!
//! let expected_steps = vec![(4, 2)];
//! assert_eq!(path.steps, expected_steps);
//! ```

use crate::node::{NodeId, NodeWeight};

#[derive(Debug, Default)]
pub struct Path<I: NodeId, W: NodeWeight> {
    pub weight: W,
    pub steps: Vec<I>,
}

impl<I: NodeId, W: NodeWeight> From<(W, Vec<I>)> for Path<I, W> {
    fn from(value: (W, Vec<I>)) -> Self {
        let (weight, steps) = value;

        Path { weight, steps }
    }
}

impl<I: NodeId, W: NodeWeight> PartialEq for Path<I, W> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && self.steps == other.steps
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::path::Path;

    #[test]
    fn test_path_struct() {
        let mut path: Path<(i32, i32), i32> = Path::default();
        println!("path: {:?}", path);

        path.steps.push((4, 2));

        let expected_steps = vec![(4, 2)];
        println!("path.steps: {:?}", path.steps);
        assert_eq!(path.steps, expected_steps);

        // path.steps.push(NodeId(4, 2));

        for step in path.steps {
            println!("step: {:?}", step);
        }

        // assert_eq!(1, 2)
    }
}
