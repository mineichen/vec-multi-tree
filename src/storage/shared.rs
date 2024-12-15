use std::cell::UnsafeCell;

use crate::{Color, Node, RedBlackTreeSet};

use super::{owned::VecStorage, InternalStorage, Storage};

pub struct SharedVecStorage<T> {
    nodes: UnsafeCell<VecStorage<T>>,
}

impl<T> SharedVecStorage<T> {
    pub fn new() -> Self {
        Self {
            nodes: VecStorage::new().into(),
        }
    }

    pub fn add_tree(&self, value: T) -> RedBlackTreeSet<&SharedVecStorage<T>> {
        let this = unsafe { &mut *self.nodes.get() };
        let root = this.len();
        let mut node: Node<_> = value.into();
        node.color = Color::Black;
        this.push(node);
        RedBlackTreeSet { nodes: self, root }
    }
}

impl<'a, T> Storage for &'a SharedVecStorage<T> {
    type Item = T;
}

impl<'a, T> InternalStorage for &'a SharedVecStorage<T> {
    fn len(&self) -> usize {
        unsafe { &*self.nodes.get() }.len()
    }

    fn push(&mut self, node: Node<Self::Item>) {
        unsafe { &mut *self.nodes.get() }.push(node)
    }
    #[cfg(test)]
    fn debug_nodes(&self) -> Vec<Node<Self::Item>>
    where
        Self::Item: Copy,
    {
        unsafe { &*self.nodes.get() }.debug_nodes()
    }

    fn debug_str(&self) -> String
    where
        Self::Item: std::fmt::Debug,
    {
        unsafe { &*self.nodes.get() }.debug_str()
    }

    fn get(&self, index: usize) -> &Node<Self::Item> {
        unsafe { &*self.nodes.get() }.get(index)
    }

    fn get_mut(&mut self, index: usize) -> &mut Node<Self::Item> {
        unsafe { &mut *self.nodes.get() }.get_mut(index)
    }
}
