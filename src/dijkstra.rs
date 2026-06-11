use crate::{
    graph::Graph,
    node::{NodeId, NodeWeight},
};

/// compute the best possible path in a graph using Dijkstra algorithm
pub fn dijkstra_path<I: NodeId, W: NodeWeight>(
    graph: &Graph<I, W>,
    start: &I,
    end: &I,
) -> Result<(), String> {
    println!("dijkstra_path function");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let graph: Graph<String, i32> = Graph::default();
        let start = "A".to_string();
        let end = "B".to_string();

        let result = dijkstra_path(&graph, &start, &end);

        // assert_eq!(result, Err("blyat".into()));
        assert_eq!(result, Ok(()));
    }
}
