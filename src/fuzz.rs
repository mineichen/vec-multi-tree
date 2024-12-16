use std::collections::BTreeSet;

use super::node::{Color, Node};
use super::storage::InternalStorage;
use super::storage::VecStorage;
use super::RedBlackTreeSet;

impl<T> RedBlackTreeSet<VecStorage<T>> {
    pub fn validate_constraints(&self) {
        let root_node = &self.nodes.get(self.root);
        assert_eq!(root_node.color, Color::Black);
        self.black_count(root_node, Color::Black);
    }
    pub(super) fn black_count(&self, node: &Node<T>, parent_color: Color) -> u16 {
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

pub(super) fn build_fuzz_tree<const LOG: bool>(
    data: &[u8],
) -> Option<RedBlackTreeSet<VecStorage<&u8>>> {
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
pub fn fuzz_insert(data: &[u8]) {
    let Some(tree) = build_fuzz_tree::<false>(data) else {
        return;
    };
    tree.validate_constraints();
    let collected = tree.iter().copied().collect::<Vec<_>>();
    let expected = data.iter().collect::<BTreeSet<_>>();
    assert_eq!(expected.len(), collected.len());
    for (a, b) in tree.iter().zip(expected.iter()) {
        assert_eq!(a, b);
    }
}
