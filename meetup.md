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

<!-- layout does not respect the prefix — bug -->

<!-- dprint-ignore -->
About me
====

<!-- cmd:list_item_newlines: 2 -->
* Data scientist @ BlueYonder
<!-- cmd:pause -->
* Robotics, machine learning and neuro-informatics 
<!-- cmd:pause -->
* Algorithms and efficient implementations
<!-- cmd:pause -->
* Tendency to over-engineering
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

- Handling indices is error-prone (e.g., out of bounds)

- Zero cost abstraction—even slightly faster than loops! 
<!-- cmd:end_slide -->




<!-- dprint-ignore -->
My problem with iterators
====

<!-- cmd:newlines: 5 -->
<!-- cmd:column_layout: [1, 1] -->
<!-- cmd:column: 1-->

<!-- alignment: center --> 
```rust {all|1,12}
use itertools::Itertools;

let window_to_window = sequence
    .iter()
    .filter_map(|i| {
        if i.is_initialized() {
            Some(i.transform())
        } else {
            None
        }
    })
    .tuple_windows()
    .collect::<HashMap<_, _>>();
```

Filter and map. Then, create a map from current to next value.

<!-- cmd:column: 0 -->
<!-- cmd:list_item_newlines: 2 -->

- Many methods on `Iterator` (`map`, `flat_map`, `filter_map`, `cycle`)
- More on `itertools` and `itermore`
- But when your use case is missing

  - Write your own iterator
  - ... just use loops

- Kompost addresses this dilemma

<!-- two column layout code example -->

<!-- cmd:speaker_note: But first -->

<!-- cmd:end_slide -->

Adding new methods to iterator
====

<!-- cmd:column_layout: [1, 1] -->
<!-- cmd:column: 0 -->

<!-- cmd:list_item_newlines: 2 -->
- You need to 

  - Pick a name (picking names is hard!)
  - Write the struct
  - Define the constructor
  - Define a trait with the function
  - Add blanket implementation to all iterators

- Repetitive boiler plate. Let's just pick a loop then or ...

<!-- cmd:column: 1 -->

```rust {1-4|6-9|10-17|18-23|25}
struct WellNamedStruct {
    // State of iteration and owned data.
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


<!-- cmd:reset_layout -->
<!-- cmd:newlines: 2 -->
<!-- alignment: center -->
... use *anonymous* iterators from `kompost`! 
<!-- cmd:end_slide -->


<!-- dprint-ignore -->
Example: Transposition of Iterators over Iterators/Slices
====

A more challenging example:

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
Transposition of iterators over iterators similar.


<!-- cmd:end_slide -->

<!-- dprint-ignore -->
Anonymous iterators as generators?
====

Anonymous iterators can emulate behaviour resembling generators
until these arrive in stable Rust:

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
let x = repeat(())
    .anonymous(|_| [1, 2, 3].into_iter(), |it| it.next())
    .collect::<Vec<_>>();
assert_eq!(x, [1, 2, 3]);
```

Can be useful to implement trees over an arena for example.

<!-- cmd:end_slide -->
Composed Iterators
====

<!-- cmd:newlines: 2 -->

`Kompost` allows you to create composite methods on `Iterator`, methods
that are an alias for parts of a method chain.

<!-- cmd:newlines: 2 -->
<!-- cmd:column_layout: [2,3] -->
<!-- cmd:column: 1 -->
```rust {1-10|12-20}
use kompost::*;

fn favorite_pipeline(
    it: impl Iterator<Item = i32>
) -> impl Iterator<Item = f64> {
    it.skip(5)
        .map(|x| x.pow(2))
        .take_while(|x| *x < 100)
        .map(|x| x as f64)
}


assert_eq!(
    [1, 2, 3, 4, 5, 6, 7]
        .into_iter()
        .composed(favorite_pipeline)
        .collect::<Vec<_>>(),
    vec![36.0f64, 49.0]
)
```

<!-- cmd:column: 0 -->

<!-- cmd:list_item_newlines: 2 -->

<!-- cmd:newlines: 3 -->

- Adds abstraction level
- reduces complexity
- Increases readability and structure
- Avoids code duplication
- Enables testing

<!-- right side: small example -->


<!-- cmd:end_slide -->
Complex composition: Circular sliding window
===
```rust {all|4|5-6|8,26|9-15|16-25}
use kompost::*;

pub fn circular_windows<T>(
    size: usize,                         // Additional parameter
    it: impl ExactSizeIterator<Item = T> + Clone,
                                         // Further restrict the iterator 
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    it.anonymous(                        // Alias for a single anonymous method
        |it| {
            let len = it.len();          // get length of the iterator
                                         // (available on ExactSizeIterator)
            (0usize, len, it.cycle())    // Context is a tuple of iteration count,
                                         // max iteration, and
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

<!-- cmd:alignment: center -->
You won't find this in any of the popular crates. It does use an index, which is limited
to this single functionality.

<!-- cmd:end_slide -->
Complex composition: Circular sliding window
===

```rust 
use kompost::*;
use kompost::composite::*;

fn main(){
  let size=x3;
  let x = [1, 2, 3, 4].into_iter()
      .composed(|i| circular_windows(size, i))
      .flatten()
      .collect::<Vec<_>>();

  println!("{x:?}");
}
```

<!-- cmd:alignment: center -->
The finished "composite" method can have a parameter but then
requires another closure.

Considering another method that accept parameters as a tuple.
<!-- cmd:pause -->

```
[1,2,3,2,3,4,3,4,1,4,1,2]
```
<!-- cmd:end_slide -->

<!-- dprint-ignore -->
Wave Function Collapse
====


<!--  cmd:alignment: center -->
<!-- cmd:column_layout: [3, 2] -->
<!-- cmd:column: 1 -->
![image:width:100%](wfc-rooms-animation.gif)

Wave function collapse (WFC) example from

https://github.com/Elwqnn/wfc
<!-- cmd:column: 0 -->

<!-- cmd:list_item_newlines: 2 -->

- Popular  level and texture generation (games)
- Rather complex algorithm

  0. First extract patterns from sample image
  1. Initialize output. All Pixels/voxels unconstrained
  2. Choose unconstrained and collapse (from distribution)
  3. Propagate, update probabilities
  4. Repeat until convergence /contradiction

- I struggled understanding the algorithm
- Focus on pattern extraction
  - Requires 2D/3D (circular) sliding windows
  - Looked at Rust implementation
<!-- cmd:end_slide -->
<!-- dprint-ignore -->
WFC For loops
====

<!-- cmd:column_layout: [1,1] -->

<!-- cmd:column: 0 -->

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

<!-- cmd:column: 1 -->
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

<!-- cmd:reset_layout -->
<!-- cmd:column_layout: [1, 1] -->
<!-- cmd:column: 0 -->
<!--  cmd:alignment: center -->
Rust code from `wfc` for Pattern extraction.
<!-- cmd:column: 1 -->
<!--  cmd:alignment: center -->
Equivalent C++ code.
<!-- cmd:reset_layout -->
<!-- pause -->
**No critique!** Works perfectly fine and is readable well enough. I just don't like loops.

<!-- cmd:end_slide -->
WFC: Two-dimensional circular, sliding windows  
====


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
<!-- alignment: center -->

* Sliding window over rows
* Within window
  * Sliding window over columns
  * Transpose. Grouping `i`-th elements

==> Iterator over Iterator over Iterator

<!-- cmd:end_slide -->

WFC: Two-dimensional circular, sliding windows  
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
    .map(|window| window.flatten().copied().collect::<Vec<_>>())
    .collect::<Vec<_>>();

println!("{r:?}");
```

<!-- alignment: center -->

Application is clean and readable.

<!-- pause -->
```
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

<!-- alignment: center -->

Only unique patterns. `Itertools` has unique.
<!-- pause -->
Can we do without?

<!-- uniqueness -->

<!-- cmd:end_slide -->

<!-- dprint-ignore -->
WFC: Pattern collections
====

```rust {all|9-14|15-20}
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

* Note powerful `collect` is and everything plays well together!
* No external crate!
* Uniqueness for each row window independently ...

<!-- cmd:end_slide -->

<!-- dprint-ignore -->
Outlook: Parallelism
====



```rust {all|1,5|1,5,12-15}
use rayon::prelude::*;

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
* Parallel version `AnonymousIterator` and `ComposedIterator`
<!-- cmd:end_slide -->

Kompost
====
<!-- newlines: 3 -->
<!-- ump_to_middle -->

<!-- cmd:column_layout: [1,1] -->
<!-- column: 1 -->
<!-- cmd:alignment: center -->
![image:width:85%](kompost.png)
Find me on GitHub!
<!-- column: 0 -->
<!-- cmd:newlines: 4 -->
<!-- cmd:list_item_newlines: 2 -->
- Anonymous iterators
- Composed iterators
- Useful composite methods (i.e., useful *to me*)


- Simple code and tested
- No macros
- No AI




