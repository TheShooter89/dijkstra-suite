//! Core computing-related types
//!
//! ## The`compute` module
//!
//! This module contains all types and logic related to computing the dijkstra algorithm itself
//! code from this module will be used primarily by `dijkstra` module in its logic
//!
//! ## Main structs
//!
//! The `DistanceFromSource` type keep tracks of each new discovered node shortest distance
//! from the source (the starting node of the inquired path) and provides the final reconstructed
//! path
//!
//! The `VisitedList` type keeps track of visited nodes as soon as the algorithm process it
//!
//! The `PriorityQueue` type implements the core pillar of implementeation, the min-heap
//! priority queue to automatically sort out fastest node connections to choose from

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Deref, DerefMut},
};

use crate::{
    node::{NodeId, NodeWeight},
    path::Path,
};

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// +++++++++++++++++++ DistanceFromSource +++++++++++++++++++
/// keep track of shortest distances discovered and reconstruct shortest path

/// ## The`DistanceFromSource` type
///
/// The `DistanceFromSource` type keep tracks of each new discovered node shortest distance
/// from the source (the starting node of the inquired path)
///
/// This type implements the concrete graph representation upon which the dijkstra
/// implementeation will run to compute the shortest path
///
/// ```rust
/// use dijkstra_suite::path::Path;
/// use dijkstra_suite::compute::DistanceFromSource;
///
/// let mut distances = DistanceFromSource::default();
///
/// distances.set_distance((0, 0), 0, None);
/// distances.set_distance((1, 0), 3, Some((0, 0)));
/// distances.set_distance((2, 0), 5, Some((0, 0)));
/// distances.set_distance((3, 0), 7, Some((1, 0)));
/// distances.set_distance((4, 0), 9, Some((3, 0)));
/// distances.set_distance((5, 0), 9, Some((3, 0)));
///
/// println!("distances: {:?}", distances);
///
/// let check_computed_path: Path<(i32, i32), i32> =
///     (9, vec![(0, 0), (1, 0), (3, 0), (5, 0)]).into();
/// let computed_path = distances.compute_path((5, 0)).unwrap();
///
/// println!("computed weight: {:?}", computed_path.weight);
/// println!("computed steps: {:?}", computed_path.steps);
/// assert_eq!(computed_path.weight, check_computed_path.weight);
/// assert_eq!(computed_path.steps, check_computed_path.steps);
/// assert_eq!(computed_path, check_computed_path);
/// ```

#[derive(Debug, Default)]
pub struct DistanceFromSource<I: NodeId, W: NodeWeight>(HashMap<I, (W, Option<I>)>);

impl<I: NodeId, W: NodeWeight> Deref for DistanceFromSource<I, W> {
    type Target = HashMap<I, (W, Option<I>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: NodeId, W: NodeWeight> DerefMut for DistanceFromSource<I, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I: NodeId, W: NodeWeight> DistanceFromSource<I, W> {
    pub fn set_distance(
        &mut self,
        id: I,
        distance: W,
        previous_node: Option<I>,
    ) -> Option<(W, Option<I>)> {
        self.insert(id, (distance, previous_node))
    }

    pub fn compute_path(&self, to: I) -> Result<Path<I, W>, String> {
        if self.get(&to).is_none() {
            return Err("end node not found".into());
        }

        let (cost, previous_node) = self.get(&to).unwrap();

        // if node exists in the list, but has no previous node
        // this node must be the starting one, so steps vec is empty
        // and cost is the same as the starting node cost
        if previous_node.is_none() {
            // let (cost, _) = self.get(&to).unwrap();

            return Ok(Path {
                weight: cost.clone(),
                steps: vec![],
            });
        }

        let mut result_path: Vec<I> = vec![];
        let result_cost = cost.clone();

        result_path.push(to);

        let mut next_node = previous_node.clone();

        while let Some(node) = next_node {
            // push the node in the resul list as a new step
            // then check for its previous node, if any
            // println!("node: {:?}", node);
            result_path.push(node.clone());

            match self.get(&node) {
                Some(step) => {
                    match &step.1 {
                        Some(next_step) => {
                            // there's a step behind this, let's
                            // push it to the result and go ahead
                            next_node = Some(next_step.clone());
                        }
                        None => {
                            // println!("we reached the start of the path");
                            next_node = None;
                        }
                    }
                }
                None => return Err("no valid path found".into()),
            }
        }

        // println!("result cost: {}", result_cost);
        // println!("result path: {:?}", result_path);

        result_path.reverse();

        Ok(Path {
            weight: result_cost,
            steps: result_path,
        })
    }
}
// +++++++++++++++++++ END DistanceFromSource +++++++++++++++++++

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// +++++++++++++++++++ VisitedList +++++++++++++++++++
/// keep track of visited nodes as soon as the algorithm process it
///
/// ## The`VisitedList` type
///
/// The `VisitedList` type keep tracks of visited nodes as soon as the algorithm process it
/// from the source (the starting node of the inquired path)
///
/// This type is basically just a wrapper around HashSet, restricted to types that implements
/// NodeId trait
///
/// ```rust
/// use dijkstra_suite::compute::VisitedList;
///
/// let node_1 = "A";
/// let node_2 = "B";
/// let node_3 = "C";
/// let node_4 = "D";
/// let node_5 = "E";
///
/// let mut visited_list = VisitedList::default();
///
/// visited_list.insert(node_4);
/// visited_list.insert(node_1);
/// visited_list.insert(node_3);
///
/// assert!(visited_list.is_visited(&node_4));
/// assert!(!visited_list.is_visited(&node_2));
/// assert!(visited_list.is_visited(&node_1));
/// ```

#[derive(Debug, Default)]
pub struct VisitedList<I: NodeId>(pub HashSet<I>);

impl<I: NodeId> VisitedList<I> {
    pub fn is_visited(&self, node_id: &I) -> bool {
        self.contains(node_id)
    }
}

impl<I: NodeId> Deref for VisitedList<I> {
    type Target = HashSet<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: NodeId> DerefMut for VisitedList<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// +++++++++++++++++++ END VisitedList +++++++++++++++++++

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// +++++++++++++++++++ PriorityQueue +++++++++++++++++++
/// min-heap queue to get the fastest connection to the next path node
///
/// ## The`PriorityQueue` type
///
/// The `PriorityQueue` type implements a priority queue using a min-heap
/// binary tree, uses [`BinaryHeap`](std::collections::BinaryHeap) under
/// the hood.
///
/// It's really just a small fancy newtype-style wrapper type around
/// [`BinaryHeap`](std::collections::BinaryHeap), restricting it to
/// `QueuedItem<T>` generic newtype to ensure proper min-like behaviour
///
/// ```rust
/// use dijkstra_suite::compute::{QueuedItem, PriorityQueue};
///
/// let node_1: i32 = 1;
/// let node_2: i32 = 2;
/// let node_3: i32 = 3;
/// let node_4: i32 = 4;
/// let node_5: i32 = 5;
///
/// let mut queue: PriorityQueue<i32> = PriorityQueue::from(vec![
///     // add explicitly trough QueuedItem
///     QueuedItem::from(node_3),
/// ]);
///
/// queue.push(node_5.into());
/// queue.push(node_2.into());
/// queue.push(node_1.into());
/// queue.push(node_4.into());
///
/// println!("peek: {:?}", queue.peek().unwrap());
///
/// assert_eq!(queue.pop().unwrap(), QueuedItem::from(1));
/// ```
pub struct PriorityQueue<T>(pub BinaryHeap<QueuedItem<T>>);

impl<T> Deref for PriorityQueue<T> {
    type Target = BinaryHeap<QueuedItem<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for PriorityQueue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Ord> From<T> for PriorityQueue<T> {
    fn from(value: T) -> Self {
        PriorityQueue(BinaryHeap::from([QueuedItem::from(value)]))
    }
}

impl<T: Ord> From<Vec<QueuedItem<T>>> for PriorityQueue<T> {
    fn from(value: Vec<QueuedItem<T>>) -> Self {
        PriorityQueue(BinaryHeap::from(value))
    }
}

// +++++++++++++++++++ END PriorityQueue  +++++++++++++++++++

// +++++++++++++++++++++++++++++++++++++++++++++++++++
// +++++++++++++++++++ QueuedItem +++++++++++++++++++
/// min-heap queue to get the fastest connection to the next path node

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct QueuedItem<T>(pub Reverse<T>);

impl<T> Deref for QueuedItem<T> {
    type Target = Reverse<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for QueuedItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for QueuedItem<T> {
    fn from(value: T) -> Self {
        QueuedItem(Reverse(value))
    }
}

// +++++++++++++++++++ END QueuedItem  +++++++++++++++++++

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distances_struct() {
        let mut distances = DistanceFromSource::default();

        let new_distance = distances.set_distance((0, 0), 0, None);
        assert_eq!(new_distance, None);

        println!("get invalid node (0, 1): {:?}", distances.get(&(0, 1)));
        assert_eq!(distances.get(&(0, 1)), None);

        // set the new value and returns the OLD value
        let new_distance = distances.set_distance((0, 0), 1, None);
        println!("new_distance: {:?}", new_distance);
        println!("distances: {:?}", distances);
        assert_eq!(new_distance, Some((0, None)));

        let last_inserted_distance = distances.get(&(0, 0));
        println!("last_inserted_distance: {:?}", last_inserted_distance);
        assert_eq!(last_inserted_distance, Some(&(1, None)));

        // assert_eq!(1, 2)
    }

    #[test]
    fn test_compute_path() {
        let mut distances = DistanceFromSource::default();

        distances.set_distance((0, 0), 0, None);
        distances.set_distance((1, 0), 3, Some((0, 0)));
        distances.set_distance((2, 0), 5, Some((0, 0)));
        distances.set_distance((3, 0), 7, Some((1, 0)));
        distances.set_distance((4, 0), 9, Some((3, 0)));
        distances.set_distance((5, 0), 9, Some((3, 0)));

        println!("distances: {:?}", distances);

        let check_computed_path = Path::default();
        let computed_path = distances.compute_path((0, 0)).unwrap();

        assert_eq!(computed_path.weight, check_computed_path.weight);
        assert_eq!(computed_path.steps, check_computed_path.steps);

        let check_computed_path: Path<(i32, i32), i32> =
            (9, vec![(0, 0), (1, 0), (3, 0), (5, 0)]).into();
        let computed_path = distances.compute_path((5, 0)).unwrap();

        println!("computed weight: {:?}", computed_path.weight);
        println!("computed steps: {:?}", computed_path.steps);
        assert_eq!(computed_path.weight, check_computed_path.weight);
        assert_eq!(computed_path.steps, check_computed_path.steps);
        assert_eq!(computed_path, check_computed_path);

        // assert_eq!(1, 2)
    }

    #[test]
    fn test_is_visited() {
        let node_1 = "A";
        let node_2 = "B";
        let node_3 = "C";
        let node_4 = "D";
        let node_5 = "E";

        let mut visited_list = VisitedList::default();

        visited_list.insert(node_4);
        visited_list.insert(node_1);
        visited_list.insert(node_3);

        assert!(visited_list.is_visited(&node_4));
        assert!(!visited_list.is_visited(&node_2));
        assert!(visited_list.is_visited(&node_1));
    }

    #[test]
    fn test_priority_queue() {
        let node_1 = "A";
        let node_2 = "B";
        let node_3 = "C";
        let node_4 = "D";
        let node_5 = "E";

        let mut queue: PriorityQueue<&str> = PriorityQueue::from(vec![
            // add explicitly trough QueuedItem
            QueuedItem::from(node_3),
            // implicitly convert to QueuedItem
            // node_2.into(),
            // node_3.into(),
            // node_4.into(),
            // node_5.into(),
        ]);

        queue.push(node_5.into());
        queue.push(node_2.into());
        queue.push(node_1.into());
        queue.push(node_4.into());

        println!("peek: {:?}", queue.peek().unwrap());

        assert_eq!(queue.pop().unwrap(), QueuedItem::from("A"));

        // assert_eq!(1, 2)
    }

    #[test]
    fn test_priority_queue_i32() {
        let node_1: i32 = 1;
        let node_2: i32 = 2;
        let node_3: i32 = 3;
        let node_4: i32 = 4;
        let node_5: i32 = 5;

        let mut queue: PriorityQueue<i32> = PriorityQueue::from(vec![
            // add explicitly trough QueuedItem
            QueuedItem::from(node_1),
            // implicitly convert to QueuedItem
            node_2.into(),
            node_3.into(),
            node_4.into(),
            node_5.into(),
        ]);

        println!("peek: {:?}", queue.peek().unwrap());

        assert_eq!(queue.pop().unwrap(), QueuedItem::from(1));

        // assert_eq!(1, 2)
    }
}
