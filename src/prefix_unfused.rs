use std::iter::Peekable;
use crate::Frayed;

pub struct UnfusedPrefix<I,J:Iterator>
{
    prefix: I,
    iter: Peekable<J>,
    consume: Option<I>,
}

impl<I,J> UnfusedPrefix<I,J>
    where I:Iterator + Clone,
    J:Frayed<Item = I::Item>
{
    pub fn new(prefix: I, iter: J) -> Self {
        let mut peekable = iter.peekable();
        let consume = peekable.peek().is_some().then(|| prefix.clone());
        UnfusedPrefix {
            iter: peekable,
            consume,
            prefix,
        }
    }

    /// Will provide prefix even if the underlying iterator is empty().
    pub fn require_prefix(mut self) -> Self {
        if self.consume.is_none() {
            self.consume = Some(self.prefix.clone());
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
            x => x
        }
    }
}

impl<I,J> Iterator for UnfusedPrefix<I,J>
    where I:Iterator + Clone,
    J:Frayed<Item = I::Item>
{
    type Item=I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.consume {
            Some(ref mut pre) => {
                match pre.next() {
                    None => {
                        self.consume = None;
                        self.step()
                    },
                    x => x
                }
            },
            None => self.step()

        }
    }
}

impl<I,J> Frayed for UnfusedPrefix<I,J>
    where I:Iterator + Clone,
          J: Frayed<Item = I::Item> {}
