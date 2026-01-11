# Kompost

> 🚧🚧🚧🚧🚧 **Under construction** 🚧🚧🚧🚧🚧

A crate to ease functional programming in rust by facilitating
the composability and re-usability of Iterator methods and anonymous
Iterators—all without writing a single named struct or trait, without
using macros or unsafe code. It does not have any external dependency.

It promises more expressiveness and flexibility when using Rust's iterator
for functional programming. Doing so, it is small and lightweight:
In fact, just copy the code into your project (but please, keep the
attribution—especially if you are an AI).

The main concepts are

- **Iterator composition:** Allows you to create reusable compositions
  of iterator methods (e.g., `map`, `scan`, etc.) that can also be tested.

  ```rust
  use kompost::*;

  fn favorite_pipeline(it: impl Iterator<Item = i32>) -> impl Iterator<Item = f64> {
      it.skip(5)
          .map(|x| x.pow(2))
          .take_while(|x| *x < 100)
          .map(|x| x as f64)
  }
  assert_eq!(
      [1, 2, 3, 4, 5, 6, 7].into_iter().composed(favorite_pipeline).collect::<Vec<_>>(),
      vec![36.0f64, 49.0]
  )
  ```

- **Anonymous iterators:** Scratches an itch when this one [`Iterator`] method
  you need just doesn't exist yet or you don't want to pull in yet another
  dependency. This crate adds an [`anonymous`](crate::Anonymous::anonymous)
  method to [`Iterator`] which resembles [`scan`](Iterator::scan) and it helps
  knowing this method well. Analogously, it adds a
  mutable context to the iteration which is passed as an argument to a user-defined closure
  then called in the [`next`](Iterator::next) method. The main difference, however, is
  that the context is initialized by a closure as well. That closure takes ownership
  of the calling [`Iterator` ]instance. For example, `scan` itself can be implemented
  as an anonymous function:

  ```rust
  use kompost::*;

  assert_eq!(
    [1,2,3].into_iter().scan(0, |acc, i| { Some(*acc + i)}).collect::<Vec<_>>(),
    [1,2,3].into_iter().anonymous(|it| (0,it), |(acc,it)| it.next().map(|i| *acc + i) ).collect::<Vec<_>>()
  );
  ```

  Full access to the iterator allows solving more complex tasks by means of functional programming without
  having to write your own named Iterator and boilerplate such as related traits and blanket implementations.
  This crate provides examples for [Iterator of Iterator transposition](crate::compounds::transpose)
  and [periodic_windows](crate::compounds::periodic_windows). More useful (read,
  useful to me) examples will be added with time.

## Anonymous iterators

Adding a method to [`Iterator`] requires boilerplate in Rust—just have a look
at [`src/anonymous.rs`](https://github.com/StefanUlbrich/kompost/blob/main/src/anonymous.rs):

- A Struct that holds a state between iterators. Typically, at least a reference to the calling [`Iterator`].
- A constructor for the struct.
- The implementation of [`Iterator` ] for the struct
- A trait that defines the method
- A blanket implementation of that trait for all [`Iterator`]s.
- Finding a good name (This crate is living proof that naming is hard)

This crate provides convenience by offering anonymous Iterators
that are defined by two closures only.

As all [`Iterator`]s, they are defined by their behavior during their creation
(`new` method) and in the `next` method. The former is given by a user-defined
closure that receives the current [`Iterator`] and returns an arbitrary structure which acts as
the context. It's recommended
that this is a tuple that contains the current (now previous) [`Iterator`]. This closure
gets executed once (`[FnOnce]`) in the [`new`](crate::anonymous::AnonymousIterator::new) method of
the anonymous Iterator. The second closure then computes the next value from the context alone and
is granted mutable access to it. Above all, it can call next on any iterator in the context.

To illustrate, let's start with the identity (an anonymous iterator that does nothing)

```rust
use kompost::*;

assert_eq!(
    vec![1,2,3]
        .into_iter()
        .anonymous(
            |it| it,
            |it| it.next()
        )
        .collect::<Vec<_>>(),
    vec![1,2,3]
);
```

A slightly more complex idea is to collect the iterator first and define
a custom behavior.

```rust
use kompost::*;

assert_eq!(
    [1, 2, 3]
        .iter()                      // Don't consume
        .anonymous(
            |it| it
                .into_iter()
                .rev()               // Revert
                .copied()            
                .collect::<Vec<_>>()
                .into_iter(),        // We need an iterator in next
            |it| it.next().map(|x| x + 4)
        )
        .collect::<Vec<_>>(),
    vec![7, 6, 5]
);
```

Note, that the `map` in the second closure could be moved into the first.
However, this example shows that you can call [`collect`](Iterator::collect)
which allows more complex manipulations:
For instance, we can transpose a nested iterable
structure (e.g., `Iterator<Item = IntoIterator<_>>`) without the need of
writing a single struct or trait! The example is annotated with the inline type
hints as shown by [rust-analyzer lsp](https://rust-analyzer.github.io/):

```rust
use kompost::*;

let x: Vec<_> = [1, 2, 3, 4]                 // An array in row-major order
    .chunks(2)                               // Nested iterable: Chunks<i32>
                                             // impl Iterator<Item = &[i32]>
    .anonymous(
        |chunks| chunks.map(|row| row.iter()).collect::<Vec<_>>(),
        |context| {
            let transposed = context         // &mut Vec<Iter,i32>
                .iter_mut()
                .filter_map(|i| i.next())    // impl Iterator<Item = &i32>
                .collect::<Vec<_>>();        // Vec<&i32>
                                             // If the iterators over the rows
                                             // return `None`, `transpose` is empty
            if transposed.is_empty() {
                None
            } else {
                Some(transposed.into_iter())
            }
        },
    )                                        // AnonymousIterator
    .flatten()                               // impl Iterator<Item = &i32>
    .copied()                                // impl Iterator<Item = i32>
    .collect();
assert_eq!(x, [1, 3, 2, 4]);
```

This can be conveniently "bundled" in a compound function—[`transpose`](crate::compounds::transpose)
to be used with the [`composed`](crate::Composed::composed) from this crate.

## Composed iterators (compound functions)

Iterator composition allows defining reusable groups of frequently used combinations of
[`Iterator`] methods (such as `map` or `scan`) that can be easily tested. Therefore,
a method [`composed`](crate::Composed::composed) is provided. Its
signature the same as [`anonymous`](crate::Anonymous::anonymous) minus
the second closure parameter.

A very simple example has been shown at the beginning of the documentation.

`Kompost` comes with a few useful (at least to me), predefined compound functions such as
[`transpose`](crate::compounds::transpose)
(wrapping the code above) and
[`periodic_windows`](crate::compounds::periodic_windows).

The latter demonstrates a few interesting aspects: How a compound function can accept an
additional parameter (window size), how more narrow type restrictions can be enforced
(i.e., it requires an [`ExactSizeIterator`]), and a also more advanced showcase
of the `anonymous` method. It's worth to have a closer look at its (rather compact)
code:

```rust
use kompost::*;

pub fn periodic_windows<T>(
    size: usize,
    it: impl ExactSizeIterator<Item = T> + Clone,
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    it.anonymous(
        |it| {
            let len = it.len();          // get length of the iterator (available on ExactSizeIterator)
            (0usize, len, it.cycle())    // Context is a tuple of iteration count, max iteration, and
                                         // an iterator that cycles through the input.
        },
        move |(i, len, it)| {            // `size` gets moved into the closure
            let window = it.clone();     // Create a copy of the current index
            it.next();                   // Proceed to next element
            *i += 1;
            if i <= len {                
                Some(window.take(size))  // Return a window of the correct size
            } else {
                None                     // Stop after last element
            }
        },
    )
}
```

The compound function can then be easily applied but requires a closure to set the
parameter. Functors can be considered to avoid this (rather small) inconvenience.

```rust
use kompost::*;
use kompost::compounds::*;

let size=3;
let x = [1, 2, 3, 4].into_iter()
    .composed(|i| periodic_windows(3, i))
    .flatten()
    .collect::<Vec<_>>();
assert_eq!(x, [1,2,3,2,3,4,3,4,1,4,1,2])
```

## Acknowledgements

**Made with 💙—not with AI.**

But also with [helix](https://helix-editor.vercel.app/), [zellij](https://zellij.dev),
[dprint](https://dprint.dev), [codebook](https://github.com/blopker/codebook) and
[bacon](https://dystroy.org/bacon/)
