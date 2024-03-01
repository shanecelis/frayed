use crate::Frayed;
use std::iter::Peekable;

pub struct Prefix<I, J: Iterator> {
    prefix: I,
    iter: Peekable<J>,
    consume: Option<I>,
}

impl<I, J> Prefix<I, J>
where
    I: Iterator + Clone,
    J: Frayed<Item = I::Item>,
{
    pub fn new(prefix: I, iter: J) -> Self {
        let mut peekable = iter.peekable();
        let consume = peekable.peek().is_some().then(|| prefix.clone());
        Prefix {
            iter: peekable,
            consume,
            prefix,
        }
    }

    /// If enabled, will provide prefix even if the underlying iterator is
    /// empty(). By default it is disabled.
    pub fn prefix_empty(mut self, enable: bool) -> Self {
        if self.consume.is_none() {
            self.consume = enable.then(|| self.prefix.clone());
        }
        self
    }

    fn step(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            None => {
                // eprintln!("cloning prefix with {} len", self.prefix.clone().count());
                self.consume = self.iter.peek().is_some().then(|| self.prefix.clone());
                None
            }
            x => x,
        }
    }
}

impl<I, J> Iterator for Prefix<I, J>
where
    I: Iterator + Clone,
    J: Frayed<Item = I::Item>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.consume {
            Some(ref mut pre) => match pre.next() {
                None => {
                    self.consume = None;
                    self.step()
                }
                x => x,
            },
            None => self.step(),
        }
    }
}

impl<I, J> Frayed for Prefix<I, J>
where
    I: Iterator + Clone,
    J: Frayed<Item = I::Item>,
{
}
