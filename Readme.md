# Kompost

> 🚧🚧🚧🚧🚧 **Warning: Under construction** but working (all examples are tested) 🚧🚧🚧🚧🚧
>

Have you ever needed a specific method on an [`Iterator`] that just did not exist
and is absent in [itertools](https://docs.rs/itertools/latest/itertools/)
and friends too? Missing a `windows` or
`circular_windows` maybe? This crate tries to help!

If enjoy working with iterators as much as I do and want to organize, bundle and test
your iterator chains,
and/or often find yourself having to write new iterators and the involved boiler
plate, then this crate might be right for you.


## Introduction

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
        [1, 2, 3]
            .into_iter()
            .scan(0, |acc, i| { Some(*acc + i) })
            .collect::<Vec<_>>(),
        [1, 2, 3]
            .into_iter()
            .anonymous(|it| (0, it), |(acc, it)| it.next().map(|i| *acc + i))
            .collect::<Vec<_>>()
    );
  ```

  Full access to the iterator allows solving more complex tasks by means of functional programming
  without having to write your own named Iterator and boilerplate such as related traits and
  blanket implementations. This crate provides examples for
  [Iterator of Iterator transposition](crate::composite::transpose)
  and [circular_windows](crate::composite::circular_windows). More useful (read,
  useful to me) examples will be added with time.

## Anonymous iterators

Adding a method to [`Iterator`] requires boilerplate in Rust—just have a look
at [`src/anonymous.rs`](https://github.com/StefanUlbrich/kompost/blob/main/src/anonymous.rs):

- A Struct that holds a state between iterators. Typically, at least a reference to the
  calling [`Iterator`].
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

This can be conveniently "bundled" in a composite function—
[`transpose`](crate::composite::transpose)
to be used with the [`composed`](crate::Composed::composed) from this crate.

**Note:** You can even use the anonymous iterator to write [`Iterator`]s that
generate output. Might come in handy until
[generators](https://dev-doc.rust-lang.org/beta/unstable-book/language-features/generators.html)
end in stable:

```rust
use kompost::*;
use std::iter::repeat;

// We need an iterator to start with. An array with an empty type `()` should work
let x = [()]
    .iter()
    .anonymous(|_| [1, 2, 3].into_iter(), |it| it.next())
    .collect::<Vec<_>>();
assert_eq!(x, [1, 2, 3]);


// Alternatively, we can save the `iter()` line above by using `repeat`.
// That's another `use` though
let x = repeat(())
    .anonymous(|_| [1, 2, 3].into_iter(), |it| it.next())
    .collect::<Vec<_>>();
assert_eq!(x, [1, 2, 3]);
```

## Composed iterators (composite functions)

Iterator composition allows defining reusable groups of frequently used combinations of
[`Iterator`] methods (such as `map` or `scan`) that can be easily tested. Therefore,
a method [`composed`](crate::Composed::composed) is provided. Its
signature the same as [`anonymous`](crate::Anonymous::anonymous) minus
the second closure parameter.

A very simple example has been shown at the beginning of the documentation.

`Kompost` comes with a few useful (at least to me), predefined composite functions such as
[`transpose`](crate::composite::transpose)
(wrapping the code above) and
[`circular_windows`](crate::composite::circular_windows).

The latter demonstrates a few interesting aspects: How a composite function can accept an
additional parameter (window size), how more narrow type restrictions can be enforced
(i.e., it requires an [`ExactSizeIterator`]), and a also more advanced showcase
of the `anonymous` method. It's worth to have a closer look at its (rather compact)
code:

```rust
use kompost::*;

pub fn circular_windows<T>(
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

The composite function can then be easily applied but requires a closure to set the
parameter. The composite function can not return a closure (i.e., be a factory) easily
as `impl` cannot be used as a return type of `FnOnce`—so the full type needs to be
written down which is often cumbersome.

Functors will be considered to avoid this (rather small) inconvenience.


```rust
use kompost::*;
use kompost::composite::*;

let size=3;
let x = [1, 2, 3, 4].into_iter()
    .composed(|i| circular_windows(3, i))
    .flatten()
    .collect::<Vec<_>>();
assert_eq!(x, [1,2,3,2,3,4,3,4,1,4,1,2])
```

## Use Case: 2D sliding windows in Wave function collapse (WFC)

The idea for this crate came to me when I was looking into
[Wave Function Collapse (WFC)](https://github.com/mxgmn/WaveFunctionCollapse) out of curiosity.
WFC is a cool algorithm to generate random output similar to a given input (typically, an image).
It can be used to generate random level in video games or even act as a simple language model.
As I struggled with the details, I had a look at the
excellent and feature-complete Rust implementation [here](https://github.com/Elwqnn/wfc).
It helped me understanding the subtleties of the algorithm I missed, but one implementation detail
caught my eye. <!-- better -->
By no means do I want to criticize the implementation, which is great—it's my tendency
for over-engineering things. In the extraction of patterns in the sample, a sliding window
over the sample image uses [four nested `for` loops and index magic](https://github.com/Elwqnn/wfc/blob/main/src/wfc.rs#L229-L238),
and I wondered whether this part could be elegantly expressed in terms of Rust [Iterator]s. 
To my surprise more complex than anticipated, and involved writing at least two new [Iterator]s resulting in a lot more boilerplate compared to
the the pragmatic approach with loops. From this observation emerged the idea for this crate
which hopefully helps in such situations.

Eventually, it turned out the sliding windows can be formulated in a rather lean and elegant way.
I want to emphasize that did not require any debugging(!)—once it compiled (which took long enough), the results were correct.
Manually dealing with indices is more error prone in my experience and might even lead to runtime out-of-bound errors.
And personally, I simply prefer declarative solutions and the ability to breakdown the problem into smaller, simple and reusable
building blocks (such as the custom [transpose](crate::composite::transpose)
and the [circular_windows](crate::composite::circular_windows) methods). I think is just a  more natural take on the algorithm
with simple steps:

* Start with a nested iteration over the memory (usually using [`chunks(number_of_columns)`](std::slice::[T]::chunks) for row-major layouts)
* Generate a (circular) windows [`Iterator`] over the *outer* [`Iterator`] (i.e., rows in row-major layouts)
* For each of the inner iterables (i.e., [`slice`]s), generate cycling windows [`Iterator`]s
  These then iterate over columns (for row-major layouts again).
* As we want to group all the first elements of these column iterators, then the second elements, then the third elements and so on,
  we need to [`transpose`](crate::composite::transpose). This is the least obvious step.
* We collect the results and that's it already. The algorithm can easily be extended to more dimensions—3D, at least, still makes sense for
  creating cave systems and other volumetric structures.

With the custom methods defined in the previous sections, this translates to:

```rust
use kompost::*;
use kompost::composite::*;

let array_2d = [1, 2, 3, 4, 5, 6, 7, 8, 9];
let (size_m, size_n) = (2, 2);
array_2d
    .chunks(3).composed(move |it| circular_windows(size_m, it))
    .map(move |rows| {
        rows.map(move |row| {
            row.into_iter()
                .composed(move |it| circular_windows(size_n.clone(), it))
        })
        .composed(transpose)
    });
    
```

This functionality is wrapped in the [`circular_windows_2d_slice`](crate::composite::circular_windows_2d_slice) and [`window_2d`](crate::composite::circular_windows_2d_slice) compositions.
It can be used on slices 

```rust
use kompost::*;
use kompost::composite::*;

let array_2d = [1, 2, 3, 4, 5, 6, 7, 8, 9];
let r = array_2d
    .chunks(3)
    .composed(|it| circular_windows_2d_slice(it, 2, 2))
    .flatten()
    .map(|window| window.flatten().copied().collect::<Vec<_>>());
assert_eq!(
    r.collect::<Vec<_>>(),
    [
        [1, 2, 4, 5],
        [2, 3, 5, 6],
        [3, 1, 6, 4],
        [4, 5, 7, 8],
        [5, 6, 8, 9],
        [6, 4, 9, 7],
        [7, 8, 1, 2],
        [8, 9, 2, 3],
        [9, 7, 3, 1],
    ]
);

```

and, again more general, on [`Iterator`] of [`Iterator`]


```rust
use kompost::*;
use kompost::composite::*;

let a = [1, 2, 3];
let b = [4, 5, 6];
let c = [7, 8, 9];
let array_2d = [a.iter(), b.iter(), c.iter()];
let r = array_2d
    .into_iter()
    .composed(|it| circular_windows_2d(it, 2, 2))
    .flatten()
    .map(|window| window.flatten().copied().collect::<Vec<_>>());
assert_eq!(
    r.collect::<Vec<_>>(),
    [
        [1, 2, 4, 5],
        [2, 3, 5, 6],
        [3, 1, 6, 4],
        [4, 5, 7, 8],
        [5, 6, 8, 9],
        [6, 4, 9, 7],
        [7, 8, 1, 2],
        [8, 9, 2, 3],
        [9, 7, 3, 1],
    ]
);
```

The WFC algorithm is of course, much more complex. It also requires finding *unique* patterns, adjacency relations between patterns,
handling symmetries and much more, which are way of of context for this section. Uniqueness, however,
is fun to integrate. With the help from a method from the [`itertools`](https://docs.rs/itertools/latest/itertools/) crate, this is already too easy.
Uniqueness is achieved by ...

```rust
use itertools::Itertools;
use kompost::*;
use kompost::composite::*;

let array_2d = [1, 1, 2, 1, 2, 1, 1, 1, 1];

let r = array_2d
    .chunks(3)
    .composed(|it| circular_windows_2d_slice(it, 2, 2))
    .flatten()
    .map(|window| window.flatten().copied().collect::<Vec<_>>())
    .unique()
    .collect::<Vec<_>>();

assert_eq!(r.len(), 6);
```

How convenient! This is why [Iterator]s are so cool. Yet, we can achieve the same without using
an external crate.
The next lines generate a [`HashSet`](std::collections::HashSet) with unique patterns for
each window over the rows. These can be collapsed into a final
single hashset that yields the unique patterns. 

```rust
use std::collections::HashSet;
use kompost::*;
use kompost::composite::*;

let array_2d = [1, 1, 2, 1, 2, 1, 1, 1, 1];
let r = array_2d
    .chunks(3)
    .composed(|it| circular_windows_2d_slice(it, 2, 2))
    .map(|row_window| {
        HashSet::<Vec<i32>>::from_iter(
            row_window.map(|window| window.flatten().copied().collect::<Vec<_>>()),
        )
    })
    .reduce(|acc, set| {
        let mut acc = acc.clone();
        acc.extend(set);
        acc.clone()
    })
    .unwrap();

assert_eq!(r.len(), 6);
```

It works but is more verbose and probably even less efficient. However, as the unique sets
are computed independently for each axis, we can consider parallelizing the computation!
For higher dimensions or large arrays, this can lead to relevant performance improvements.

[`Rayon`](https://docs.rs/rayon/latest/rayon/) is the first choice for parallelization in Rust and works well with concept of
iteration. `Kompost` does not support rayon yet so this will be the next thing to do!

## Acknowledgements

**Made with 💙—not with AI.**

But also with [helix](https://helix-editor.vercel.app/), [zellij](https://zellij.dev),
[dprint](https://dprint.dev), [codebook](https://github.com/blopker/codebook) and
[bacon](https://dystroy.org/bacon/)
