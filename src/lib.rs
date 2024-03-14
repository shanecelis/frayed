#![doc(html_root_url = "https://docs.rs/frayed/0.1.0")]
#![doc = include_str!("../README.md")]
pub mod fraught;
pub mod defray;
pub use defray::Defray;
use fraught::prefix::Prefix;

/// Marker trait
pub trait Frayed: Iterator {}

/// Adapter struct
pub struct FrayedIter<I: Iterator> {
    pub unfused: I,
}

/// Frayed tools only operate on iterators marked as `Frayed`.
pub trait FrayedTools: Frayed {
    /// Turn a frayed iterator into an iterator of iterator, that is, de-fray
    /// it.
    fn defray(self) -> Defray<Self>
    where
        Self: Sized,
    {
        Defray::new(self)
    }
}

impl<T> FrayedTools for T where T: Frayed + ?Sized {}

/// Fraught tools operate on regular iterators but may accept or return `Frayed`
/// iterators.
pub trait FraughtTools: Iterator {
    /// Use this iterator as a prefix for a frayed iterator with many postfixes.
    fn prefix<I>(self, postfixes: I) -> Prefix<Self, I>
    where
        I: Frayed<Item = Self::Item>,
        Self: Sized + Clone,
    {
        Prefix::new(self, postfixes)
    }

    /// Mark this iterator as `Frayed`. Can then use the `FrayedTools` extension
    /// methods.
    fn frayed(self) -> FrayedIter<Self>
    where
        Self: Sized,
    {
        FrayedIter { unfused: self }
    }
}

impl<T> FraughtTools for T where T: Iterator + ?Sized {}

impl<I: Iterator> Iterator for FrayedIter<I> {
    type Item = I::Item;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.unfused.next()
    }
}

impl<I: Iterator> Frayed for FrayedIter<I> {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }
