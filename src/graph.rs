use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::node::{Node, NodeId, NodeWeight};

#[derive(Debug)]
pub struct Graph<I: NodeId, W: NodeWeight>(HashMap<I, Node<I, W>>);

impl<I: NodeId, W: NodeWeight> Deref for Graph<I, W> {
    type Target = HashMap<I, Node<I, W>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: NodeId, W: NodeWeight> DerefMut for Graph<I, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<I: NodeId, W: NodeWeight> PartialEq for Graph<I, W> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<I: NodeId, W: NodeWeight> Graph<I, W> {
    pub fn shortest_path(self) -> Vec<I> {
        //

        vec![]
    }
}
