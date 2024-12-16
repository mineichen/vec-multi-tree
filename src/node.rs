use crate::key::OptionKey;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Color {
    Red,
    Black,
}

// Type is public but only referenced in trait, which is sealed
#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub(crate) value: T,
    pub(crate) color: Color,
    pub(crate) parent: OptionKey,
    pub(crate) left: OptionKey,
    pub(crate) right: OptionKey,
}

impl<T> Node<T> {
    #[inline(always)]
    pub fn is_right(&self, key: usize) -> bool {
        debug_assert!(key != usize::MAX);
        self.right == key
    }
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            color: Color::Red,
            parent: Default::default(),
            left: Default::default(),
            right: Default::default(),
        }
    }
}
