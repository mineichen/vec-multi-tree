use std::fmt::Debug;

use crate::{Color, Node};

pub struct VecStorage<T> {
    nodes: Vec<Node<T>>,
}

impl<T> VecStorage<T> {
    pub fn new(value: T) -> Self {
        let mut node: Node<_> = value.into();
        node.color = Color::Black;
        Self { nodes: vec![node] }
    }

    // Returns number of nodes from all trees and deleted nodes.
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn push(&mut self, node: Node<T>) {
        self.nodes.push(node)
    }

    #[cfg(test)]
    pub(crate) fn iter(&mut self) -> impl Iterator<Item = &'_ Node<T>> {
        self.nodes.iter()
    }

    pub(crate) fn debug_str(&self) -> String
    where
        T: Debug,
    {
        self.nodes
            .iter()
            .map(|x| format!("{x:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[inline(always)]
    pub(crate) fn get(&self, index: usize) -> &Node<T> {
        #[cfg(debug_assertions)]
        {
            &self.nodes[index]
        }
        /// Safety: Is using only indices created by this library
        #[cfg(not(debug_assertions))]
        unsafe {
            self.nodes.get_unchecked(index)
        }
    }
    #[inline(always)]
    pub(crate) fn get_mut(&mut self, index: usize) -> &mut Node<T> {
        #[cfg(debug_assertions)]
        {
            &mut self.nodes[index]
        }

        /// Safety: Is using only indices created by this library
        #[cfg(not(debug_assertions))]
        unsafe {
            self.nodes.get_unchecked_mut(index)
        }
    }
}
