# Frayed

Unfused and unashamed iterators under the hood, conventional iterators out the
door

# Introduction

Rust iterators[^0] come in a few varieties: fused, unfused, and now frayed. The variety
is determined by how it behaves after `.next()` returns `None`.

```
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```


## Fused

A fused iterator once `.next()` returns `None` will only ever return `None`. It
is said to be exhausted at the first `None`. This behavior can be guaranteed
with the [`.fuse()`][fuse] method that returns a [std::iter::Fuse][Fuse] iterator.

## Unfused

The majority of iterators are unfused. They have no programmatic guarantees like
[Fuse][Fuse] but the producers and consumers have tacitly agreed that it's
impolite to ask `.next()` of an iterator who has already said `None`. It is
expected to be exhausted at the first `None`.

## Frayed

Frayed iterators delight in furnishing further elements after returning `None`.
They can economically represent multiple sequences. They are not indefatigably
barbaric, however; a frayed iterator is expected to be exhausted when it returns
two `None`s consecutively.

# Usage

Suppose we have an iterator that represents multiple sequences. For instance
`SevenIter`is a unfused iterator that represents these sequences: `[1, 2]`, `[4,
5]`, and `[7]`.

```compile rust
use frayed::{Frayed, FrayedTools, FraughtTools};

/// This iterator skips every number divisible by 3 up to seven.
struct SevenIter(u8);

/// SevenIter's .next() returns Some(1), Some(2), None, Some(4), Some(5), None, 
/// Some(7), None, None, and so on.
impl Iterator for SevenIter {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        self.0 += 1;
        (self.0 % 3 != 0 && self.0 <= 7).then_some(self.0)
    }
}

/// Mark iterator with `.frayed() or impl the `Frayed` marker trait.
let frayed_iter = SevenIter(0).frayed();
/// Defray the frayed iterator into an iterator of iterators.
for subiter in &frayed_iter.defray() {
    for i in subiter {
        print!("{i} ");
    }
    println!()
}
```

The above will produce this output

``` text
1 2 
4 5
7 
```

# Extensions 

## FrayedTools

`FrayedTools` extends iterators marked `Frayed`. 

## FraughtTools

`FraughtTools` extends regular iterators but often either accepts frayed
iterators as arguments or returns frayed iterators.

# Q&A

## Why restrict FrayedTools to only Frayed iterators? 

Because dealing with an iterator that conceptually represents many iterators is
confusing: Sometimes you want to map over all the elements. Sometimes you want
to map over the subsequences. This way the compiler can help us.

For instance if we forget to mark our iterator as "frayed", we can't "defray" it.

``` ignore rust
let frayed_iter = SevenIter(0); // .frayed();
let _ = &frayed_iter.defray(); // Not marked frayed. No `defray()` method.
```

## Defray isn't an iterator. How can I "map" over it?

`Defray` implements `IntoIterator` for `&Defray`. The problems appear when one
wants to return a `Defray` but say `.map()` its output. If one can do what they
need with the underlying frayed iterator, use `defrayed.into_inner()` to
retrieve it, process it, and consider `.defray()`-ing it again.

If one wants to process the iterators and not the underlying elements, that
remains an open question as to how best to do that. Perhaps `Defray` itself
could provide a `map<F,Y>(self, f: F) -> Map<Defray<Iter<Item=X>>, F, Y> where
F: FnMut(Iter<Item=X>) -> Y)` function.

# Motivation

When writing an iterator implementation by hand that represents multiple
sequences, it is usually easy to write a `Iterator<Item = Vec<T>>` even though
allocating and collecting a `Vec` is not essential or desired. However, writing a
`Iterator<Item = SubIter<T>>` requires tricky machinery[^1] if one hopes to
respect the API that entails, e.g., the `SubIter`s can be consumed out of order
or dropped.

If one writes instead a "frayed" iterator `Iterator<Item = T>`---where the
`None` represents the end of a subsequence not the end of the iterator---that is
often much easier. One can consume these iterators with a some care but they
remain unconventional and surprising.

```compile rust
fn raw_consume_unfused<T: std::fmt::Display>(frayed: impl Iterator<Item = T>) {
    let mut frayed = frayed.peekable();
    loop {
        // Consume the subsequence.
        for i in frayed.by_ref() {
            print!("{} ", i);
        }
        println!();
        // Check for second None that means frayed iterator is exhausted.
        if frayed.peek().is_none() {
            break;
        }
    }
}
```

The initial motivation of this crate is to make it easy for "frayed" iterators
to be consumed by the uninitiated. Consider instead this code:

```compile rust
use frayed::*;
fn raw_consume_frayed<T: std::fmt::Display>(frayed: impl Iterator<Item = T> + Frayed) {
    for subiter in &frayed.defray() {
        for i in subiter {
            print!("{} ", i);
        }
    }
}
```

But it would be even better if producers kept their frayed iterators under the
covers and then exposed the abstractions that we're all used to.

```ignore rust
fn chunks(&self) -> frayed::Defray<Iter> {
    // ...
}

fn main() {
    let obj = ...;
    for subiter in &obj.chunks() {
        for i in subiter {
            print!("{} ", i);
        }
    }
}

```




[Fuse]: https://doc.rust-lang.org/std/iter/struct.Fuse.html
[fuse]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse

[^0]: Rust iterators have a wonderfully succinct trait. 

[^1]: See [GroupBy](https://docs.rs/itertools/latest/itertools/structs/struct.GroupBy.html) implementation in itertools for an example.
