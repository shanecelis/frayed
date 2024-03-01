mod prefix_unfused;
mod split_unfused;
use crate::prefix_unfused::UnfusedPrefix;
use crate::split_unfused::SplitUnfused;

/// Marker trait
pub trait Frayed: Iterator { }

/// Adapter struct
pub struct FrayedIter<I: Iterator> { pub unfused: I }

/// Frayed tools only operate on iterators marked as `Frayed`.
pub trait FrayedTools: Frayed {
    /// Turn a frayed iterator into an iterator of iterator, that is, "chunks"
    /// that are no longer frayed.
    fn chunk(self) -> SplitUnfused<Self> where Self: Sized {
        split_unfused::new(self)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
