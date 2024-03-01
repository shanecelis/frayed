# Frayed

Unfused and unashamed iterators

# Introduction

Iterators come in many varieties:

- fused
- unfused
- frayed

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

[Fuse]: https://doc.rust-lang.org/std/iter/struct.Fuse.html
[fuse]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse
