use crate::{
    graph::Graph,
    node::{NodeId, NodeWeight},
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
        Ok(Path::default())
    }
}
