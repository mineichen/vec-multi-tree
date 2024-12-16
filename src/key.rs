//! Option<usize> would be too inefficient... use magic-value usize::MAX for null
//! This could only be achieved, if the vec contains usize::MAX elements, which is not possible, as removal just unlinks items
use core::fmt::Debug;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub(super) struct OptionKey(usize);

impl Debug for OptionKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0 == usize::MAX {
            f.debug_tuple("None").finish()
        } else {
            f.debug_tuple("Some").field(&self.0).finish()
        }
    }
}

impl PartialEq<usize> for OptionKey {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl OptionKey {
    #[inline(always)]
    pub const fn none() -> Self {
        Self(usize::MAX)
    }

    #[inline(always)]
    pub const fn new(x: usize) -> Self {
        debug_assert!(x != usize::MAX);
        Self(x)
    }

    #[inline(always)]
    pub const fn get(&self) -> Option<usize> {
        if self.0 == usize::MAX {
            None
        } else {
            Some(self.0)
        }
    }

    #[inline(always)]
    pub fn insert_if_none(&mut self, x: usize) -> bool {
        debug_assert!(x != usize::MAX);
        if self.0 == usize::MAX {
            self.0 = x;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub const fn unwrap(&self) -> usize {
        debug_assert!(self.0 != usize::MAX);
        self.0
    }
}
impl Default for OptionKey {
    fn default() -> Self {
        Self::none()
    }
}
