use dijkstra_suite::{dijkstra_path, graph::Graph};

extern crate dijkstra_suite;

fn main() {
    let graph: Graph<String, i32> = Graph::default();
    let start = "A".to_string();
    let end = "B".to_string();

    let _shortest_path = dijkstra_path(&graph, start, end);
}
