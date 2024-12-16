use super::{InternalRefStorage, InternalStorage, Storage};
use crate::{node::Color, node::Node};
use alloc::vec::Vec;

pub struct VecStorage<T>(Vec<Node<T>>);

impl<T> VecStorage<T> {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn new_with(value: T) -> Self {
        let mut node: Node<_> = value.into();
        node.color = Color::Black;
        Self(alloc::vec![node])
    }
}

impl<T> Storage for VecStorage<T> {
    type Item = T;
}

impl<T> InternalStorage for VecStorage<T> {
    // Returns number of nodes from all trees and deleted nodes.
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, node: Node<T>) {
        self.0.push(node)
    }

    #[cfg(test)]
    fn debug_nodes(&self) -> Vec<Node<T>>
    where
        T: Copy,
    {
        self.0.clone()
    }
    #[cfg(any(feature = "fuzz", test))]
    fn debug_str(&self) -> String
    where
        Self::Item: std::fmt::Debug,
    {
        self.0
            .iter()
            .map(|x| format!("{x:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[inline(always)]
    fn get(&self, index: usize) -> &Node<T> {
        #[cfg(debug_assertions)]
        {
            &self.0[index]
        }
        /// Safety: Is using only indices created by this library
        #[cfg(not(debug_assertions))]
        unsafe {
            self.0.get_unchecked(index)
        }
    }
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> &mut Node<T> {
        #[cfg(debug_assertions)]
        {
            &mut self.0[index]
        }

        /// Safety: Is using only indices created by this library
        #[cfg(not(debug_assertions))]
        unsafe {
            self.0.get_unchecked_mut(index)
        }
    }
}

impl<T> InternalRefStorage for VecStorage<T> {}
