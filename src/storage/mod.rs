use std::{
    cell::UnsafeCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{Color, Node, RedBlackTreeSet};

mod owned;
mod shared;

pub use owned::VecStorage;
pub use shared::SharedVecStorage;

// pub struct OwnedVecStorage<T>(VecStorage<T>);

// impl<T> Deref for OwnedVecStorage<T> {
//     type Target = VecStorage<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for OwnedVecStorage<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

pub trait Storage {
    type Item;
}

// Todo: Make create internal
pub trait InternalStorage: Storage {
    fn len(&self) -> usize;
    fn push(&mut self, node: Node<Self::Item>);
    #[cfg(test)]
    fn debug_nodes(&self) -> Vec<Node<Self::Item>>
    where
        Self::Item: Copy;
    fn debug_str(&self) -> String
    where
        Self::Item: Debug;
    fn get(&self, index: usize) -> &Node<Self::Item>;
    fn get_mut(&mut self, index: usize) -> &mut Node<Self::Item>;
}
