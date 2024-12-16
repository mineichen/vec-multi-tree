// Iter struct to allow in-order traversal

use super::key::OptionKey;
use super::storage::{InternalStorage, Storage};
use super::RedBlackTreeSet;

pub struct Iter<'a, TStorage> {
    tree: &'a RedBlackTreeSet<TStorage>,
    next: OptionKey,
}

impl<'a, TStorage: 'a + InternalStorage> RedBlackTreeSet<TStorage>
where
    <TStorage as Storage>::Item: 'a + Ord,
{
    /// Safety: References musten't be accessible in safe code, if TStorage doesn't implement InternalRefStorage
    pub(crate) unsafe fn create_iterator(&self) -> Iter<'_, TStorage> {
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
