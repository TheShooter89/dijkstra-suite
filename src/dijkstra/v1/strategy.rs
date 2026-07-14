use std::collections::BinaryHeap;

use crate::{
    compute::{DistanceFromSource, PriorityQueue, QueuedItem, VisitedList},
    graph::Graph,
    node::{NodeConnection, NodeId, NodeWeight},
    path::Path,
    strategy::ImplementationStrategy,
};

fn generic_zero<I: NodeId, W: NodeWeight>(graph: &Graph<I, W>, from: &I) -> W {
    // first distance is 0, but we do not know exactly "what 0 is" in the
    // context of generic W: NodeWeight type, so we "create" the 0 subtracting
    // start node weight from itself (therefore zero by elision: if weight is 3, 3-3 = 0)
    // this works because NodeWeight types implements Sub trait
    let start_weight = &graph.get(&from).unwrap().weight.clone();
    let opposite_weight = start_weight.clone();
    let start_weight = opposite_weight.clone();
    start_weight - opposite_weight
}

fn init_distances<I: NodeId, W: NodeWeight>(
    graph: &Graph<I, W>,
    distances_list: &mut DistanceFromSource<I, W>,
    start_node: &I,
) {
    let zero = generic_zero(graph, start_node);
    distances_list.set_distance(start_node.clone(), zero, None);
}

#[derive(Debug)]
pub struct DijkstraAlgorithm {}

impl ImplementationStrategy for DijkstraAlgorithm {
    type Opts = ();

    fn run<I: NodeId, W: NodeWeight>(
        graph: &Graph<I, W>,
        from: I,
        to: I,
        options: Self::Opts,
    ) -> Result<Path<I, W>, String> {
        println!("from: {:#?}", from);
        println!("to: {:#?}", to);

        println!("end node: {:#?}", graph.get(&to).take());

        let mut result_path: Path<I, W> = Path::default();

        let mut visited_nodes: VisitedList<I> = VisitedList::default();
        let mut node_distances: DistanceFromSource<I, W> = DistanceFromSource::default();
        let mut queue: PriorityQueue<NodeConnection<I, W>> = PriorityQueue(BinaryHeap::new());

        init_distances(graph, &mut node_distances, &from);
        visited_nodes.insert(from.clone());

        println!(
            "[iteration: {:#?}] starting distances: {:#?}",
            from, node_distances
        );

        let start_node_neighbours = graph.get(&from).unwrap().neighbours.clone();

        for connection in start_node_neighbours {
            queue.push(QueuedItem::from(connection));
        }
        println!(
            "[iteration: {:#?}] queue after start node inserts: {:#?}",
            from, queue
        );

        visited_nodes.insert(from.clone());

        println!(
            "[iteration: {:#?}] visited_nodes after start node inserts: {:#?}",
            from, visited_nodes
        );

        let mut current_node = from.clone();

        while let Some(connection) = queue.pop() {
            //
            current_node = connection.item().from.clone();
            println!(
                "[iteration: {:#?}] fastest connection from queue: {:#?}",
                current_node, connection
            );

            let item = connection.item().clone();
            node_distances.set_distance(item.to.clone(), item.weight, Some(item.from));

            println!(
                "[iteration: {:#?}] node_distances after new distance push: {:#?}",
                current_node, node_distances
            );

            if item.to == to {
                break;
            }

            let neighbours = graph.get(&item.to).unwrap().neighbours.clone();

            for neighbour in neighbours {
                if visited_nodes.is_visited(&neighbour.to) {
                    continue;
                }

                let mut new_connection = neighbour;
                let last_conn_weight = connection.item().weight.clone();
                new_connection.weight = new_connection.weight + last_conn_weight;

                queue.push(QueuedItem::from(new_connection));
            }
        }

        println!("[END] final node_distances: {:#?}", node_distances);

        let fastest_path = node_distances.compute_path(to);
        println!("[END] fastest_path: {:#?}", fastest_path);

        fastest_path
    }
}

#[cfg(test)]
mod test {
    use crate::{
        graph::Graph,
        node::{Node, NodeConnection},
        path::Path,
        strategy::Strategy,
        v1::strategy::DijkstraAlgorithm,
    };

    #[test]
    fn test_dijkstra_v1_strategy() {
        let mut graph: Graph<&str, i32> = Graph::default();
        let node_a = Node {
            id: "A",
            weight: 0,
            neighbours: vec![
                NodeConnection {
                    from: "A",
                    to: "B",
                    weight: 3,
                },
                NodeConnection {
                    from: "A",
                    to: "C",
                    weight: 5,
                },
                NodeConnection {
                    from: "A",
                    to: "D",
                    weight: 9,
                },
            ],
        };

        let node_b = Node {
            id: "B",
            weight: 0,
            neighbours: vec![
                NodeConnection {
                    from: "B",
                    to: "A",
                    weight: 3,
                },
                NodeConnection {
                    from: "B",
                    to: "C",
                    weight: 3,
                },
                NodeConnection {
                    from: "B",
                    to: "D",
                    weight: 4,
                },
                NodeConnection {
                    from: "B",
                    to: "E",
                    weight: 7,
                },
            ],
        };

        let node_c = Node {
            id: "C",
            weight: 0,
            neighbours: vec![
                NodeConnection {
                    from: "C",
                    to: "A",
                    weight: 5,
                },
                NodeConnection {
                    from: "C",
                    to: "B",
                    weight: 3,
                },
                NodeConnection {
                    from: "C",
                    to: "D",
                    weight: 2,
                },
                NodeConnection {
                    from: "C",
                    to: "E",
                    weight: 6,
                },
                NodeConnection {
                    from: "C",
                    to: "F",
                    weight: 8,
                },
            ],
        };

        let node_d = Node {
            id: "D",
            weight: 0,
            neighbours: vec![
                NodeConnection {
                    from: "D",
                    to: "A",
                    weight: 9,
                },
                NodeConnection {
                    from: "D",
                    to: "B",
                    weight: 4,
                },
                NodeConnection {
                    from: "D",
                    to: "C",
                    weight: 2,
                },
                NodeConnection {
                    from: "D",
                    to: "E",
                    weight: 2,
                },
                NodeConnection {
                    from: "D",
                    to: "F",
                    weight: 2,
                },
            ],
        };

        let node_e = Node {
            id: "E",
            weight: 0,
            neighbours: vec![
                NodeConnection {
                    from: "E",
                    to: "B",
                    weight: 7,
                },
                NodeConnection {
                    from: "E",
                    to: "C",
                    weight: 6,
                },
                NodeConnection {
                    from: "E",
                    to: "D",
                    weight: 2,
                },
                NodeConnection {
                    from: "E",
                    to: "F",
                    weight: 5,
                },
            ],
        };

        let node_f = Node {
            id: "F",
            weight: 0,
            neighbours: vec![
                NodeConnection {
                    from: "F",
                    to: "C",
                    weight: 8,
                },
                NodeConnection {
                    from: "F",
                    to: "D",
                    weight: 2,
                },
                NodeConnection {
                    from: "F",
                    to: "E",
                    weight: 5,
                },
            ],
        };

        graph.insert(node_a.id, node_a);
        graph.insert(node_b.id, node_b);
        graph.insert(node_c.id, node_c);
        graph.insert(node_d.id, node_d);
        graph.insert(node_e.id, node_e);
        graph.insert(node_f.id, node_f);

        let result = Strategy::execute::<DijkstraAlgorithm, &str, i32>(&graph, "A", "F");

        let expected_path: Path<&str, i32> = Path {
            weight: 9,
            steps: vec!["A", "B", "D", "F"],
        };

        // assert_eq!(1, 2);
        assert_eq!(result.unwrap(), expected_path)
    }

    #[test]
    fn test_dijkstra_v1_strategy_float() {
        let mut graph: Graph<&str, f32> = Graph::default();
        let node_a = Node {
            id: "A",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "A",
                    to: "B",
                    weight: 7.0,
                },
                NodeConnection {
                    from: "A",
                    to: "E",
                    weight: 1.0,
                },
            ],
        };

        let node_b = Node {
            id: "B",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "B",
                    to: "A",
                    weight: 7.0,
                },
                NodeConnection {
                    from: "B",
                    to: "C",
                    weight: 3.0,
                },
                NodeConnection {
                    from: "B",
                    to: "E",
                    weight: 8.0,
                },
            ],
        };

        let node_c = Node {
            id: "C",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "C",
                    to: "B",
                    weight: 3.0,
                },
                NodeConnection {
                    from: "C",
                    to: "D",
                    weight: 6.0,
                },
                NodeConnection {
                    from: "C",
                    to: "E",
                    weight: 2.0,
                },
            ],
        };

        let node_d = Node {
            id: "D",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "D",
                    to: "C",
                    weight: 6.0,
                },
                NodeConnection {
                    from: "D",
                    to: "E",
                    weight: 7.0,
                },
            ],
        };

        let node_e = Node {
            id: "E",
            weight: 0.0,
            neighbours: vec![
                NodeConnection {
                    from: "E",
                    to: "A",
                    weight: 1.0,
                },
                NodeConnection {
                    from: "E",
                    to: "B",
                    weight: 8.0,
                },
                NodeConnection {
                    from: "E",
                    to: "C",
                    weight: 2.0,
                },
                NodeConnection {
                    from: "E",
                    to: "D",
                    weight: 7.0,
                },
            ],
        };

        graph.insert(node_a.id, node_a);
        graph.insert(node_b.id, node_b);
        graph.insert(node_c.id, node_c);
        graph.insert(node_d.id, node_d);
        graph.insert(node_e.id, node_e);

        let result = Strategy::execute::<DijkstraAlgorithm, &str, f32>(&graph, "B", "D");

        let expected_path: Path<&str, f32> = Path {
            weight: 9.0,
            steps: vec!["B", "C", "D"],
        };

        // assert_eq!(1, 2);
        assert_eq!(result.unwrap(), expected_path)
    }
}
