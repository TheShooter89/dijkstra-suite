use std::collections::BinaryHeap;

use crate::{
    compute::{DistanceFromSource, PriorityQueue, VisitedList},
    graph::Graph,
    node::{NodeConnection, NodeId, NodeWeight},
    path::Path,
    strategy::ImplementationStrategy,
};

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
        // let mut queue: PriorityQueue<NodeConnection<I, W>> = PriorityQueue(BinaryHeap::new());

        Ok(Path::default())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        graph::Graph,
        node::{Node, NodeConnection},
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

        // println!("graph: {:#?}", graph);
        println!("graph.get('D'): {:#?}", graph.get("D"));

        let _result = Strategy::execute::<DijkstraAlgorithm, &str, i32>(&graph, "A", "F");
        // let result = Strategy::execute::<DijkstraAlgorithm, i8, i8>(&Graph::default(), 0, 0);

        assert_eq!(1, 2)
    }
}
