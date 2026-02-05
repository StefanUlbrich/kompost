/*
* Copyright (c) 2026 Stefan Ulbrich
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of this software
* and associated documentation files (the “Software”), to deal in the Software without restriction,
* including without limitation the rights to use, copy, modify, merge, publish, distribute,
* sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all copies or
* substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
* BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
* NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
* DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
* OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

// TODO better name for composite
//! Collection of composite  method that useful (to me)
use crate::{Anonymous, Composed};

// TODO rename to transpose_slices (including in the documentation!)
// Todo move all examples to the readme
/// Function to be used with the [`crate::Composed::composed`] method.
/// It transposes an [`IntoIterator`] of [`std::slice`], a data structure often encountered
/// when storing 2D arrays in a single (row-major) array and using
/// [`chunks`](slice::chunks) for iteration
///
/// ## Example
///
/// ```
/// # use kompost::{Composed, composite::transpose_slice};
/// let x: Vec<_> = [1, 2, 3, 4]                 // An array in row-major order
///     .chunks(2)                               // Only defined on slices and vectors
///     .composed(transpose_slice)
///     .collect();
/// assert_eq!(x, [1, 3, 2, 4]);
/// ```
pub fn transpose_slice<'a, T: 'a + Copy>(
    iter: impl Iterator<Item = &'a [T]>,
) -> impl Iterator<Item = T> {
    iter.into_iter()
        .anonymous(
            |chunks| chunks.map(|row| row.iter()).collect::<Vec<_>>(),
            |context| {
                let transposed = context // &mut Vec<Iter,i32>
                    .iter_mut()
                    .filter_map(Iterator::next) // impl Iterator<Item = &i32>
                    // .filter_map(|i| i.next()) // impl Iterator<Item = &i32>
                    .collect::<Vec<_>>(); // Vec<&i32>
                // If the iterators over the rows return `None`, transpose is empty
                if transposed.is_empty() {
                    None
                } else {
                    Some(transposed.into_iter())
                }
            },
        ) // AnonymousIterator
        .flatten() // impl Iterator<Item = &i32>
        .copied() // impl Iterator<Item = i32>
}

// TODO rename to transpose (the general version).
/// TBD: Transpose over Iterable of Iterable
///
/// ```rust
/// use kompost::*;
/// use kompost::composite::*;
///
/// let a = [1, 2, 3];
/// let b = [4, 5, 6];
/// let c = [a.iter(), b.iter()];
/// let d = c
///     .into_iter()
///     .composed(transpose)
///     .flatten()
///     .copied()
///     .collect::<Vec<_>>();
/// assert_eq!(d, [1,4,2,5,3,6]);
/// ```
pub fn transpose<T>(
    iter: impl Iterator<Item = impl Iterator<Item = T>>,
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    iter.into_iter()
        .anonymous(std::iter::Iterator::collect::<Vec<_>>, |context| {
            let transposed = context
                .iter_mut()
                .filter_map(Iterator::next)
                .collect::<Vec<_>>();
            if transposed.is_empty() {
                None
            } else {
                Some(transposed.into_iter())
            }
        })
}

/// A composite  function to be used with the [`crate::Composed::composed`] method that takes
/// an additional single `usize` as a parameter and computes a window of that size for *every element*
/// of the iterator (circular, it takes elements from the beginning for later windows).
///
/// This is requires to write an additional closure when it is used, but this might change in the future
/// when a functor trait might be written instead.
///
/// ## Example
///
/// ```
/// # use kompost::{Composed, composite::circular_windows};
/// let size=3;
/// let x = [1, 2, 3, 4].into_iter()
///     .composed(|i| circular_windows(3, i))
///     .flatten()
///     .collect::<Vec<_>>();
/// assert_eq!(x, [1,2,3,2,3,4,3,4,1,4,1,2])
/// ```
pub fn circular_windows<T>(
    size: usize,
    it: impl ExactSizeIterator<Item = T> + Clone,
) -> impl Iterator<Item = impl Iterator<Item = T>> {
    (0..it.len()).map(move |i| it.clone().chain(it.clone()).skip(i).take(size))
}

//TODO move example to Readme and link to document
/// composite  function to generate circular sliding windows over a 2D data structure
/// in form of an [`Iterator`] over slices (such as returned by the [`chunks`](slice::chunks) method)
/// See [this example](crate#complex-example section) for how to use it.
pub fn circular_windows_2d_slice<'a, T: 'a>(
    it: impl ExactSizeIterator<Item = &'a [T]> + Clone,
    size_m: usize,
    size_n: usize,
) -> impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = &'a T>>>> {
    it.composed(move |it| circular_windows(size_m, it))
        .map(move |rows| {
            rows.map(move |row| row.iter().composed(move |it| circular_windows(size_n, it)))
                .composed(transpose)
        })
}

/// composite  function to generate circular sliding windows over a 2D data structure
/// in form of an [`Iterator`] over [`Iterator`].
/// See [this example](crate#complex-example section) for how to use it.
pub fn circular_windows_2d<T>(
    it: impl ExactSizeIterator<Item = impl ExactSizeIterator<Item = T> + Clone> + Clone,
    size_m: usize,
    size_n: usize,
) -> impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = impl Iterator<Item = T>>>> {
    it.composed(move |it| circular_windows(size_m, it))
        .map(move |rows| {
            rows.map(move |row| row.composed(move |it| circular_windows(size_n, it)))
                .composed(transpose)
        })
}
