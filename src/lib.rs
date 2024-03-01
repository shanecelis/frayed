//! Iterators come in many varieties:
//!
//! - fused
//! - unfused
//! - frayed
//!
//! # Fused
//!
//! A fused iterator once `.next()` returns `None` will only ever return `None`.
//! It is exhausted at the first `None`. This behavior can be guaranteed with
//! the `.fuse()` method that returns a [std::iter::Fuse] iterator.
//!
//! # Unfused
//!
//! The majority of iterators are unfused. They have no programmatic guarantees
//! like [Fuse] but the producers and consumers have tacitly agreed that it's
//! impolite to ask `.next()` of an iterator who has already said `None`.
//!
//! # Frayed
//!
//! Frayed iterators delight in returning elements after their first, second, or
//! even third `None`. They can economically be used to represent many
//! sequences. We are not barbarians, however, so we consider a frayed iterator
//! to be exhausted when it returns two `None`s consecutively.
//!
pub mod frayed;
pub mod fraught;
use crate::fraught::prefix::UnfusedPrefix;
use crate::frayed::split::{self, SplitUnfused};

/// Marker trait
pub trait Frayed: Iterator { }

/// Adapter struct
pub struct FrayedIter<I: Iterator> { pub unfused: I }

/// Frayed tools only operate on iterators marked as `Frayed`.
pub trait FrayedTools: Frayed {
    /// Turn a frayed iterator into an iterator of iterator, that is, "chunks"
    /// that are no longer frayed.
    fn chunk(self) -> SplitUnfused<Self> where Self: Sized {
        split::new(self)
    }
}

/// Fraught tools operate on regular iterators but may accept or return `Frayed`
/// iterators.
pub trait FraughtTools: Iterator {
    /// Use this iterator as a prefix for a frayed iterator with many postfixes.
    fn prefix<I>(self, postfixes: I) -> UnfusedPrefix<Self, I>
    where I: Frayed<Item = Self::Item>,
          Self: Sized + Clone
    {
        UnfusedPrefix::new(self, postfixes)
    }

    /// Mark this iterator as `Frayed`. Can then use the `FrayedTools` extension
    /// methods.
    fn frayed(self) -> FrayedIter<Self> where Self: Sized {
        FrayedIter { unfused: self }
    }
}

impl<T> FraughtTools for T where T: Iterator + ?Sized {}
impl<T> FrayedTools for T where T: Frayed + ?Sized {}

impl<I: Iterator> Iterator for FrayedIter<I> {
    type Item = I::Item;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.unfused.next()
    }
}

impl<I: Iterator> Frayed for FrayedIter<I> { }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }
