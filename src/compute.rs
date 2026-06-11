use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    node::{NodeId, NodeWeight},
    path::Path,
};

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
}
