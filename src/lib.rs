#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::cmp::Ordering;

use key::OptionKey;
use node::{Color, Node};
use storage::{InternalStorage, Storage};

mod key;
mod node;
mod storage;

#[cfg(feature = "alloc")]
pub use storage::SharedVecStorage;

pub struct RedBlackTreeSet<TStorage> {
    nodes: TStorage,
    root: usize,
}

#[cfg(feature = "alloc")]
impl<T: Ord> RedBlackTreeSet<storage::VecStorage<T>> {
    pub fn new(value: T) -> Self {
        RedBlackTreeSet {
            nodes: storage::VecStorage::new_with(value),
            root: 0,
        }
    }
}

impl<TStorage: InternalStorage> RedBlackTreeSet<TStorage>
where
    <TStorage as Storage>::Item: Ord,
{
    pub fn insert(&mut self, value: <TStorage as Storage>::Item) -> usize {
        // Create new node
        let new_node_idx = self.nodes.len();
        let mut new_node = Node {
            value,
            color: Color::Red,
            parent: OptionKey::none(),
            left: OptionKey::none(),
            right: OptionKey::none(),
        };

        // If tree is empty, set as root and color black
        let mut current = self.root;
        loop {
            match self.compare_node_value(current, &new_node.value) {
                Ordering::Less => {
                    let cur_node = self.nodes.get_mut(current);
                    if cur_node.right.insert_if_none(new_node_idx) {
                        new_node.parent = OptionKey::new(current);
                        break;
                    }
                    current = cur_node.right.unwrap();
                }
                Ordering::Greater => {
                    let cur_node = self.nodes.get_mut(current);
                    if cur_node.left.insert_if_none(new_node_idx) {
                        new_node.parent = OptionKey::new(current);
                        break;
                    }
                    current = cur_node.left.unwrap();
                }
                Ordering::Equal => {
                    // If equal, we could either replace or keep existing
                    // Here we're choosing to keep existing
                    return current;
                }
            }
        }

        self.nodes.push(new_node);
        self.insert_fixup(new_node_idx);

        new_node_idx
    }

    fn insert_fixup(&mut self, mut node: usize) {
        while let Some(parent_idx) = self.nodes.get(node).parent.get() {
            //println!("Fixup {node}");
            // If parent is black, tree is valid
            if self.nodes.get(parent_idx).color == Color::Black {
                break;
            }

            // Get grandparent (must exist if parent is red)
            let grandparent_idx = self.nodes.get(parent_idx).parent.unwrap();
            let is_parent_right = self.nodes.get(grandparent_idx).is_right(parent_idx);

            let uncle_idx = if !is_parent_right {
                self.nodes.get(grandparent_idx).right
            } else {
                self.nodes.get(grandparent_idx).left
            };

            // Uncle red case
            if let Some(uncle_idx) = uncle_idx
                .get()
                .filter(|&idx| self.nodes.get(idx).color == Color::Red)
            {
                self.nodes.get_mut(parent_idx).color = Color::Black;
                self.nodes.get_mut(uncle_idx).color = Color::Black;
                self.nodes.get_mut(grandparent_idx).color = Color::Red;
                node = grandparent_idx;
                continue;
            }

            let is_node_right = self.nodes.get(parent_idx).is_right(node);
            // Rotation cases
            match (is_parent_right, is_node_right) {
                (true, true) => {
                    let parent_index = self.nodes.get(node).parent.unwrap();
                    self.nodes.get_mut(parent_index).color = Color::Black;
                    self.nodes.get_mut(grandparent_idx).color = Color::Red;
                    self.rotate_left(grandparent_idx);
                }
                (true, false) => {
                    node = parent_idx;
                    self.rotate_right(node);
                }
                (false, true) => {
                    node = parent_idx;
                    self.rotate_left(node);
                }
                (false, false) => {
                    let parent_index = self.nodes.get(node).parent.unwrap();
                    self.nodes.get_mut(parent_index).color = Color::Black;
                    self.nodes.get_mut(grandparent_idx).color = Color::Red;
                    self.rotate_right(grandparent_idx);
                }
            }
        }

        // Ensure root is always black
        self.nodes.get_mut(self.root).color = Color::Black;
    }

    fn compare_node_value(&self, node_idx: usize, value: &<TStorage as Storage>::Item) -> Ordering {
        self.nodes.get(node_idx).value.cmp(value)
    }

    fn rotate_left(&mut self, node_idx: usize) {
        // println!("Rotate left {node_idx}");
        let right_child_idx = self.nodes.get(node_idx).right.unwrap();

        // Update parent references
        self.nodes.get_mut(right_child_idx).parent = self.nodes.get(node_idx).parent;

        if let Some(parent_idx) = self.nodes.get(node_idx).parent.get() {
            let parent_node = self.nodes.get_mut(parent_idx);
            if parent_node.left == node_idx {
                parent_node.left = OptionKey::new(right_child_idx);
            } else {
                parent_node.right = OptionKey::new(right_child_idx);
            }
        } else {
            self.root = right_child_idx;
        }

        // Rotate
        self.nodes.get_mut(node_idx).right = self.nodes.get(right_child_idx).left;
        if let Some(left_of_right) = self.nodes.get(right_child_idx).left.get() {
            self.nodes.get_mut(left_of_right).parent = OptionKey::new(node_idx);
        }
        self.nodes.get_mut(right_child_idx).left = OptionKey::new(node_idx);
        self.nodes.get_mut(node_idx).parent = OptionKey::new(right_child_idx);
    }

    fn rotate_right(&mut self, node_idx: usize) {
        //  println!("Rotate right");
        let left_child_idx = self.nodes.get(node_idx).left.unwrap();

        // Update parent references
        self.nodes.get_mut(left_child_idx).parent = self.nodes.get(node_idx).parent;
        if let Some(parent_idx) = self.nodes.get(node_idx).parent.get() {
            let parent_node = self.nodes.get_mut(parent_idx);
            if parent_node.right == node_idx {
                parent_node.right = OptionKey::new(left_child_idx);
            } else {
                parent_node.left = OptionKey::new(left_child_idx);
            }
        } else {
            self.root = left_child_idx;
        }

        // Rotate
        self.nodes.get_mut(node_idx).left = self.nodes.get(left_child_idx).right;
        if let Some(right_of_left) = self.nodes.get(left_child_idx).right.get() {
            self.nodes.get_mut(right_of_left).parent = OptionKey::new(node_idx);
        }
        self.nodes.get_mut(left_child_idx).right = OptionKey::new(node_idx);
        self.nodes.get_mut(node_idx).parent = OptionKey::new(left_child_idx);
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, TStorage>
    where
        <TStorage as Storage>::Item: 'a,
    {
        self.into_iter()
    }

    pub fn find(&self, value: &<TStorage as Storage>::Item) -> Option<usize> {
        let mut current = self.root;

        loop {
            match self.compare_node_value(current, value) {
                Ordering::Equal => {
                    return Some(current);
                }
                Ordering::Less => {
                    current = self.nodes.get(current).right.get()?;
                }
                Ordering::Greater => {
                    current = self.nodes.get(current).left.get()?;
                }
            }
        }
    }
}

// Iter struct to allow in-order traversal

pub struct Iter<'a, TStorage> {
    tree: &'a RedBlackTreeSet<TStorage>,
    next: OptionKey,
}

impl<'a, TStorage: 'a + InternalStorage> IntoIterator for &'a RedBlackTreeSet<TStorage>
where
    <TStorage as Storage>::Item: 'a + Ord,
{
    type Item = &'a <TStorage as Storage>::Item;
    type IntoIter = Iter<'a, TStorage>;

    fn into_iter(self) -> Self::IntoIter {
        // Start with the root if it exists and is active
        let mut current = self.root;
        while let Some(x) = self.nodes.get(current).left.get() {
            current = x;
        }

        Iter {
            tree: self,
            next: OptionKey::new(current),
        }
    }
}

impl<'a, TStorage: 'a + InternalStorage> Iterator for Iter<'a, TStorage>
where
    <TStorage as Storage>::Item: Ord + 'a,
{
    type Item = &'a <TStorage as Storage>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.next.get()?;
        let node = &self.tree.nodes.get(current);

        // Get the value
        let value = &node.value;

        // Prepare next node in the iteration
        self.next = match node.right.get() {
            Some(mut x) => {
                while let Some(k) = &self.tree.nodes.get(x).left.get() {
                    x = *k;
                }
                OptionKey::new(x)
            }
            None => {
                let mut parent = node.parent;
                while let Some((k, parent_node)) = parent.get().map(|k| (k, self.tree.nodes.get(k)))
                {
                    if parent_node.right == current {
                        current = k;
                        parent = parent_node.parent;
                    } else {
                        break;
                    }
                }
                parent
            }
        };

        Some(value)
    }
}

#[cfg(any(feature = "fuzz", test))]
impl<T> RedBlackTreeSet<storage::VecStorage<T>> {
    pub fn validate_constraints(&self) {
        let root_node = &self.nodes.get(self.root);
        assert_eq!(root_node.color, Color::Black);
        self.black_count(root_node, Color::Black);
    }
    fn black_count(&self, node: &Node<T>, parent_color: Color) -> u16 {
        if parent_color == Color::Red && node.color == Color::Red {
            panic!("Two subsequent RED nodes");
        }
        (match (node.left.get(), node.right.get()) {
            (None, None) => 0,
            (None, Some(right)) => self.black_count(self.nodes.get(right), node.color),
            (Some(left), None) => self.black_count(self.nodes.get(left), node.color),
            (Some(left), Some(right)) => {
                let left_count = self.black_count(self.nodes.get(left), node.color);
                let right_count = self.black_count(self.nodes.get(right), node.color);
                assert_eq!(left_count, right_count);
                left_count
            }
        }) + (node.color == Color::Black) as u16
    }
}

#[cfg(any(feature = "fuzz", test))]
fn build_fuzz_tree<const LOG: bool>(
    data: &[u8],
) -> Option<RedBlackTreeSet<storage::VecStorage<&u8>>> {
    let mut iter = data.iter();
    let first = iter.next()?;
    let mut tree = RedBlackTreeSet::new(first);
    for x in data {
        #[cfg(test)]
        if LOG {
            println!();
        }
        tree.insert(x);
        if LOG {
            println!("Root: {}\n{}", tree.root, tree.nodes.debug_str())
        }
    }
    if LOG {
        println!("Done inserting");
    }
    Some(tree)
}
#[cfg(any(feature = "fuzz", test))]
pub fn fuzz_insert(data: &[u8]) {
    let Some(tree) = build_fuzz_tree::<false>(data) else {
        return;
    };
    tree.validate_constraints();
    let collected = tree.iter().copied().collect::<Vec<_>>();
    let expected = data.iter().collect::<std::collections::BTreeSet<_>>();
    assert_eq!(expected.len(), collected.len());
    for (a, b) in tree.iter().zip(expected.iter()) {
        assert_eq!(a, b);
    }
}

#[cfg(test)]
mod tests {
    use super::node::Node;
    use super::*;

    #[test]
    fn rotate_right() {
        let mut tree = RedBlackTreeSet::new(15);

        // Insert some values
        tree.insert(5);
        tree.insert(1);
        tree.validate_constraints();

        assert_eq!(
            vec![Color::Red, Color::Black, Color::Red],
            tree.nodes
                .debug_nodes()
                .iter()
                .map(|x| x.color)
                .collect::<Vec<_>>()
        );
        assert_eq!(vec![1, 5, 15], tree.iter().copied().collect::<Vec<_>>());
    }

    #[test]
    fn share_storage() {
        let storage = storage::SharedVecStorage::new();
        let mut tree = storage.add_tree(1);
        tree.insert(0);

        let mut tree2 = storage.add_tree(2);
        tree2.insert(0);
        assert_eq!(vec![&0, &2], tree2.iter().collect::<Vec<_>>());
        assert_eq!(vec![&0, &1], tree.iter().collect::<Vec<_>>())
    }

    #[test]
    fn complex() {
        let mut tree = RedBlackTreeSet::new(5);
        tree.insert(8);
        tree.insert(9);
        tree.insert(12);
        tree.insert(13);
        tree.insert(15);
        tree.insert(19);
        tree.insert(23);
        tree.insert(10);

        tree.validate_constraints();

        assert_eq!(
            vec![
                &Node {
                    color: Color::Black,
                    value: 5,
                    parent: OptionKey::new(1),
                    left: OptionKey::none(),
                    right: OptionKey::none(),
                },
                &Node {
                    color: Color::Red,
                    value: 8,
                    parent: OptionKey::new(3),
                    left: OptionKey::new(0),
                    right: OptionKey::new(2),
                },
                &Node {
                    color: Color::Black,
                    value: 9,
                    parent: OptionKey::new(1),
                    left: OptionKey::none(),
                    right: OptionKey::new(8),
                },
                &Node {
                    color: Color::Black,
                    value: 12,
                    parent: OptionKey::none(),
                    left: OptionKey::new(1),
                    right: OptionKey::new(5),
                },
                &Node {
                    color: Color::Black,
                    value: 13,
                    parent: OptionKey::new(5),
                    left: OptionKey::none(),
                    right: OptionKey::none(),
                },
                &Node {
                    color: Color::Red,
                    value: 15,
                    parent: OptionKey::new(3),
                    left: OptionKey::new(4),
                    right: OptionKey::new(6),
                },
                &Node {
                    color: Color::Black,
                    value: 19,
                    parent: OptionKey::new(5),
                    left: OptionKey::none(),
                    right: OptionKey::new(7),
                },
                &Node {
                    color: Color::Red,
                    value: 23,
                    parent: OptionKey::new(6),
                    left: OptionKey::none(),
                    right: OptionKey::none(),
                },
                &Node {
                    color: Color::Red,
                    value: 10,
                    parent: OptionKey::new(2),
                    left: OptionKey::none(),
                    right: OptionKey::none(),
                },
            ],
            tree.nodes.debug_nodes().iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn insert_big_small_middle() {
        build_fuzz_tree::<true>(&[203, 47, 65]);
        fuzz_insert(&[203, 47, 65]);
    }

    #[test]
    fn insert_big_small_smaller() {
        build_fuzz_tree::<true>(&[203, 47, 10]);
        fuzz_insert(&[203, 47, 10]);
    }
    #[test]
    fn fuzzer_fail() {
        build_fuzz_tree::<true>(&[37, 1, 0, 219]);
        fuzz_insert(&[37, 1, 0, 219]);
    }
}
