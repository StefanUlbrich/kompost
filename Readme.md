# Kompost

🚧🚧🚧🚧🚧 **Under construction** 🚧🚧🚧🚧🚧

A crate to ease functional programming in rust by facilitating
the composability and re-usability of Iterator methods and anonymous
Iterators—all without writing any trait or struct, without using macros
or unsafe code.

## Anonymous iterators

Adding a method to `Iterator` requires boilerplate in Rust—just have a look
at [`src/anonymous.rs`]:

- A Struct that holds a state (e.g., the parent iterator)
- A Constructor
- The implementation of `Iterator` for the struct
- A trait that defines the method
- A blanket implementation of that trait for all `Iterator`s.

This crate provides convenience by offering anonymous Iterators
that are defined by two closures only.

As all `Iterator`s, they are defined by their behavior during creation and in
the next method. The former is given by a closure that receives the current
[`Iterator`] and returns a context structure. The second closure
then computes the next value from the context and has mutable access to this context.
Hence, the `anonymous()` method strongly resembles `Iterator::scan()` with the notable
exception that the first parameter is a closure.

```rust
use kompost::*;

assert_eq!(
    vec![1,2,3]
        .into_iter()
        .anonymous(
            |it| it.into_iter(),
            |it| it.next()
        )
        .collect::<Vec<_>>(),
    vec![1,2,3]
);
```

A slightly more complex idea is to collect the iterator first and define
a custom behavior.

```rust
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

This enables more complex tasks. For instance, we can transpose a nested iterable
structure ([`IntoIterator`]`<Item =` [`IntoIterator`]`<_>>`) without the need of
writing a single struct or trait! The example is annotated with the inline type
hints as shown by [rust-analyzer lsp](https://rust-analyzer.github.io/):

```rust
let x: Vec<_> = [1, 2, 3, 4]                 // An array in row-major order
    .chunks(2)                               // Nested iterable: Chunks<i32>
    .anonymous(
        |chunks| chunks.map(|row| row.iter()).collect::<Vec<_>>(),
        |context| {
            let transposed = context         // &mut Vec<Iter,i32>
                .iter_mut()
                .filter_map(|i| i.next())    // impl Iterator<Item = &i32>
                .collect::<Vec<_>>();        // Vec<&i32>
                                             // If the iterators over the rows return `None`, transpose is empty
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

This can be conveniently "bundled" in a function—[`transpose()`](crate::transpose)
to be used with the [`composed()`](crate::ComposedIterable::composed) from this crate.

## Composed iterators

Iterator composition allows defining reusable groups of frequently used combinations of [`Iterator`]
methods (such as `map` or `scan`) that can be easily tested.

A very simple example might look like:

```rust
use kompost::ComposedIterable;
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

`Kompost` comes with a few useful (at least to me), predefined compound functions such as
`transpose` and `periodic_windows()`.

The letter shows a few interesting aspects: How a compound function can accept an additional parameter,
how stricter type restrictions can be enforced (`ExactSizeIterator`), and a more advanced showcase
of the `anonymous()` method.

```rust
pub fn periodic_windows<T>(
    size: usize,
    it: impl ExactSizeIterator<Item = T> + Clone,
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    it.anonymous(
        |it| {
            let len = it.len();
            (0usize, len, it.cycle())
        },
        move |(i, len, it)| {
            let window = it.clone();
            it.next();
            *i += 1;
            if i <= len {
                Some(window.take(size))
            } else {
                None
            }
        },
    )
}
```

The compound function can then be easily applied:

```rust
let size=3;
let x = [1, 2, 3, 4].into_iter()
    .composed(|i| periodic_windows(3, i))
    .flatten()
    .collect::<Vec<_>>();
assert_eq!(x, [1,2,3,2,3,4,3,4,1,4,1,2])
```
