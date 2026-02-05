---
title: "**Kompost**"
sub_title: Anonymous and composed iterators
date: 2026-02-05
author: Stefan Ulbrich
theme:
  name: tokyonight-storm
options:
  command_prefix: "cmd:"
---

<!-- layout does not respect the prefixe — bug -->

<!-- dprint-ignore -->
About me
====

<!-- cmd:list_item_newlines: 2 -->
* Data scientist @ BlueYonder
<!-- cmd:pause -->
* Robotics, machine learning and neuroinformatics 
<!-- cmd:pause -->
* Algorithms and efficient implementations
<!-- cmd:pause -->
* Tendency to overengineering
<!-- as you will see -->

<!-- cmd:end_slide -->

<!-- dprint-ignore -->
Teaser
====

<!--  cmd:alignment: center -->
<!-- cmd:column_layout: [3, 2] -->
<!-- cmd:column: 1 -->
![image:width:100%](wfc-rooms-animation.gif)

Wave Front Collapse (WFC)
<!-- cmd:column: 0 -->
<!-- cmd:alignment: center -->
```c++
for (int y : iota(0, y_max)) {
    for (int x : iota(0, x_max)) {
        vector<Pixel> pixels;
        pixels.reserve(n * n);

        // or cartesian_product
        for (int dy : iota(0, n)) {
            for (int dx : iota(0, n)) {
                pixels.push_back(
                  sample->get(
                    (x + dx) % sample->width,
                    (y + dy) % sample->height
                ));
            }
        }
        
        Pattern pattern(n, move(pixels));
    }
}
```

C++


<!-- cmd:end_slide -->

<!-- dprint-ignore -->
Iterators in Rust
===

<!-- cmd:newlines: 5 -->
<!-- cmd:column_layout: [1, 1] -->
<!-- cmd:column: 1-->


```rust {1|1-2|1-3|1-4|1-5|1-6|1-7|1-8|1-9}
let winner: S = sequence
    .iter()
    .skip_while(|i| !i.is_initialized())
    .filter(|s| s.is_valid())
    .zip(sequence2.iter())
    .map(|(s1, s2)| s1.take_best(s2))
    .cycle()
    .take_while(|s| s.getting_bored())
    .reduce(|a, b| a.take_best(&b));
```

<!-- cmd:column: 0 -->
<!-- cmd:list_item_newlines: 2 -->
- C++ iterators are *cursors*
- Rust iterators are *streams*

- Iterators are *functional programming*
- They are declarative and readable

- They are slightly faster than loops
- Handling indices is error-prone (e.g., out of bounds)

- Zero cost—even slightly faster! 
<!-- cmd:end_slide -->




<!-- dprint-ignore -->
My problem with iterators
====

- Example with `windows`
- Itertools
- What I need is missing, and then its hard

- Kompost addresses this

<!-- two column layout code example -->

<!-- cmd:speaker_note: But first -->

<!-- cmd:end_slide -->

Adding new methods to iterator
====

<!-- cmd:column_layout: [1, 1] -->
<!-- cmd:column: 0 -->
- You need to write

  - Pick a name (the name "Kompost" is living proof that picking names is hard)
  - The iterator (Code with elipses)
  - Add a constructor
  - Define a trait with the function
  - Blanket implementation of the trait for all iterators

- Tedious. Let's just pick a loop then ... or

<!-- cmd:column: 1 -->

```rust {1-4|6-9|10-17|18-23|25}
struct WellNamedStruct {
    // State of iteraton and owned data.
    // E.d., current index
}

impl WellNamedStruct {
  fn new() -> Self { Self { /* .. */ }}
}

impl Iterator for WellNamedStruct {
    type Item = usize; 

    fn next(&mut self) -> Option<Self::Item> {
        // Some(42) to continue, None to stop
    }
}

trait WellNamedTrait: Iterator {
    fn well_named_method(self) -> WellNamedStruct
    {
        WellNamedStruct::new()
    }
}

impl<I: Iterator> WellnamedTrait for I {}
```


<!-- cmd:end_slide -->



<!-- dprint-ignore -->
Anonymous iterators
====


<!-- cmd:newlines: 3 -->

<!-- cmd:alignment: center -->
<!-- cmd:column_layout: [1,1] -->

<!-- cmd:column: 0 -->



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



<!-- cmd:column: 1 -->

```rust {4-7|8-13}
  use kompost::*;

  assert_eq!(
      [1, 2, 3]
          .into_iter()
          .scan(0, |acc, i| Some(*acc + i) )
          .collect::<Vec<_>>(),
      [1, 2, 3]
          .into_iter()
          .anonymous(
              |it| (0, it),
              |(acc, it)|
                  it.next().map(|i| *acc + i))
          .collect::<Vec<_>>()
  );
```

<!-- cmd:alignment: center -->
<!-- cmd:column_layout: [1,1] -->

<!-- cmd:column: 0 -->
Identity
<!-- cmd:column: 1 -->
Comparison to `scan`


<!-- cmd:end_slide -->


<!-- dprint-ignore -->
Example: Transposition of Iterators over Iterators/Slices
====

```rust {all|3,4|7|9-19|20-24|25} 
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
<!-- cmd:end_slide -->



<!-- dprint-ignore -->
Anonymous Iterators as Generators?
====

```rust {all|4-9|11-20}
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

<!-- cmd:end_slide -->
Composed Iterators
====

- Testing
- Avoid code duplication
- Readability and structure

<!-- right side: small example -->

<!-- On the exampple of transpose -->

<!-- cmd:end_slide -->
Composion: Sliding window
===
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

<!-- cmd:end_slide -->
Composion: Sliding window
===

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
<!-- cmd:end_slide -->
Composed Iteratprs
====

<!-- cmd:end_slide -->
<!-- dprint-ignore -->
Kompost
====

# Additions 

## Composed Iterators

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

<!-- cmd:end_slide -->

Kompost
====

## Anonumous Iterators

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

<!-- cmd:end_slide -->
<!-- dprint-ignore -->
WFC For loops
====

```rust
for y in 0..y_max {
    for x in 0..x_max {
        let mut pixels = Vec::with_capacity(n * n);
        for dy in 0..n {
            for dx in 0..n {
                let sx = (x + dx) % sample.width;
                let sy = (y + dy) % sample.height;
                pixels.push(sample.get(sx, sy));
            }
        }
        let pattern = Pattern::new(n, pixels);
        // ...
    }
}      
```

```c++
for (int y : iota(0, y_max)) {
    for (int x : iota(0, x_max)) {
        vector<Pixel> pixels;
        pixels.reserve(n * n);
        
        for (int dy : iota(0, n)) {
            for (int dx : iota(0, n)) {
                pixels.push_back(sample->get((x + dx) % sample->width,
                                             (y + dy) % sample->height));
            }
        }
        
        Pattern pattern(n, move(pixels));
    }
}
```

- image (gif?)
- how it works

  - extract patterns from a smaple image
  - Pick unconstrained and collapse
  - propagate

- Pattern extraction

  - Requires a 2D sliding window
  - how it looks in the `wfc` crate
    - what I don't like
    - challenge to do it better


<!-- cmd:end_slide -->
WFC: Two-dimensional circular, sliding windos  
====

<!-- Mention earlier that the reason for kompost becomes iminent later (here) -->
<!-- Two column, text on the left --> 

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

<!-- cmd:end_slide -->

WFC: Two-dimensional circular, sliding windos  
====

<!-- run example -->

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

<!-- cmd:end_slide -->
<!-- dprint-ignore -->
WFC: Pattern collections
====

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
<!-- uniqueness -->

<!-- cmd:end_slide -->

<!-- dprint-ignore -->
WFC: Pattern collections
====

```rust {all|9-11|13-16}
use std::collections::HashSet;
use kompost::*;
use kompost::composite::*;

let array_2d = [1, 1, 2, 1, 2, 1, 1, 1, 1];  // 6 unique patterns
let patterns = array_2d
    .chunks(3)
    .composed(|it| circular_windows_2d_slice(it, 2, 2))
    .map(|row_window| {
        HashSet::<Vec<i32>>::from_iter(
            row_window.map(|window|
              window.flatten().copied().collect::<Vec<_>>()),
        )
    })
    .fold(HashSet::new(), |mut acc, set| {
        acc.extend(set);
        acc
    });
```

* Note how well iterators interact!
* No external crate!
* Uniqueness for each row window independently ...

<!-- cmd:end_slide -->

<!-- dprint-ignore -->
Outlook: Parallelism
====



```rust {all|1,5|1,5,12-15}
use rayoun::prelude::*;

let array_2d = [1, 1, 2, 1, 2, 1, 1, 1, 1];
let patterns = array_2d
    .par_chunks(3)
    .composed(|it| circular_windows_2d_slice(it, 2, 2))
    .map(|row_window| {
        HashSet::<Vec<i32>>::from_iter(
            row_window.map(|window|
              window.flatten().copied().collect::<Vec<_>>()),
        )
    })
    .fold(HashSet::new(), |mut acc, set| {    // Will cause problems
        acc.extend(set);
        acc
    });

```
* Parallelism with Rayon!
* Requires implementing `IntoParIterator`
* Parallel version `AnonymousIterator` and `ComposedIterar`
<!-- cmd:end_slide -->

Kompost
====

<!-- ump_to_middle -->
![image:width:25%](kompost.png)


- No macros
- Simple code and tested
- Useful composite methods (i.e., useful *to me*)

Find me on GitHub!




<!-- cmd:end_slide -->


   - Iteration in C++ (loops) 

   - Example of a for loop in C++ 
   - Iteration example 
     - functional programming 
     - Easier to read (opinionated, depends on domain) compared to indices! 
       Maybe code from `wfc` without explanation 
     - Reminds me of pandas method chaining. 
   - But what if something is missing? 
     - so many `map`, `flatten`, `flat_map`, `scan`, `filter`, `filter_map` 
     - and the "terminations", `reduce`, `fold` (**important**: use fold in readme), `collect` (amazing) 
     - But sliding windows? circular sliding windows? 2D operations? Domain 

   - Repetitions? Testing? Solution is: iterators are 
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
