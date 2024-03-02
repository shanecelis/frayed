# Frayed

Unfused and unashamed iterators

# Introduction

Iterators come in few varieties: fused, unfused, and now frayed.

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

Frayed iterators delight in returning elements after the first, second, or even
third `None`. They can economically be used to represent many sequences. They
are not indefatigably barbaric, however; a frayed iterator is said to be
exhausted when it returns two `None`s consecutively.

# Motivation

When writing an iterator implementation by hand that represents multiple
sequences, it's usually easy to write a `Iterator<Item = Vec<T>>` even though
allocating and collecting a `Vec` isn't essential or even desired. However,
writing a `Iterator<Item = SubIter<T>>` can require some pretty tricky machinery
if one hopes to respect the total API that entails, e.g., the `SubIter` can be
consumed out of order or dropped. 

If I write a "frayed" iterator `Iterator<Item = T>`, where the `None` represents
the end of a subsequence, that's often not difficult. One can consume these
iterators with a little care but they remain unconventional. The initial
motivation of this crate was to make it easy for "frayed" iterators to be
consumed by the uninitiated.

# Usage

Suppose we have an iterator that represents multiple subsequences.

```compile rust
use frayed::{Frayed, FrayedTools, FraughtTools};

struct SevenIter(u8);
/// This iterator skips every number divisible by 3 up to seven.
/// SevenIter(0) returns 1, 2, None, 3, 4, None, 7, None, None, ...
impl Iterator for SevenIter {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        self.0 += 1;
        (self.0 % 3 != 0 && self.0 <= 7).then_some(self.0)
    }
}
// Mark iterator as `Frayed`. Can also use `.frayed()` method.
// impl Frayed for SevenIter {}

/// Mark iterator as frayed. Can also impl the `Frayed` marker trait.
let frayed_iter = SevenIter(0).frayed();
for subiter in &frayed_iter.chunk() {
    for i in subiter {
        print!("{i} ");
    }
    println!()
}
```

`FrayedTools` has extension methods for iterators marked `Frayed`. Why restrict
these extension to only frayed iterators? Because dealing an iterator that
conceptually represents a bunch of iterators can be confusing. This way the
compiler can help us. 

For instance if we forget to mark our iterator as "frayed", we can't "chunk" it.

``` ignore rust
let frayed_iter = SevenIter(0); // .frayed();
let _ = &frayed_iter.chunk(); // Not marked frayed. No `chunk()` method.
```



[Fuse]: https://doc.rust-lang.org/std/iter/struct.Fuse.html
[fuse]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse
