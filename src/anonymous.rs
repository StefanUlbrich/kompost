/*
* Copyright (c) <year> <copyright holders>
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

use std::marker::PhantomData;

/// Iterator that with a context and a closures called in the [`AnonymousIterator::new`] and
/// [`AnonymousIterator::next`] methods.
pub struct AnonymousIterator<Iter, In, Out, Init, Next, Context>
where
    Init: FnOnce(Iter) -> Context,
    Next: Fn(&mut Context) -> Option<Out>,
    Iter: Iterator<Item = In>,
{
    // init_fn: Init,
    next_fn: Next,
    context: Context,
    _iter: PhantomData<Iter>,
    _init: PhantomData<Init>,
}

impl<Iter, In, Out, Init, Next, Context> AnonymousIterator<Iter, In, Out, Init, Next, Context>
where
    Init: FnOnce(Iter) -> Context,
    Next: Fn(&mut Context) -> Option<Out>,
    Iter: Iterator<Item = In>,
{
    pub fn new(iter: Iter, init_fn: Init, next_fn: Next) -> Self {
        Self {
            next_fn,
            context: init_fn(iter),
            _iter: PhantomData,
            _init: PhantomData,
        }
    }
}

impl<Iter, In, Out, Init, Next, Context> Iterator
    for AnonymousIterator<Iter, In, Out, Init, Next, Context>
where
    Init: FnOnce(Iter) -> Context,
    Next: Fn(&mut Context) -> Option<Out>,
    Iter: Iterator<Item = In>,
{
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        (self.next_fn)(&mut self.context)
    }
}

/// Trait that adds a method to [`IntoIterator`] structs (i.e., [`Iterator`]s, [`slice`](std::slice)s, and [`Vec`]s) that
/// allows painlessly create anonynmous iterators for one-time use.
pub trait AnonymouslyIterable<Iter, In, Out, Init, Next, Context>
where
    Init: FnOnce(Iter) -> Context,
    Next: Fn(&mut Context) -> Option<Out>,
    Iter: Iterator<Item = In>,
{
    /// Creates an anonymous iterator.
    ///
    /// The iterator is defined by its behavior upon creation ([`AnonymousIterator::new`]) and when
    /// [`AnonymousIterator::next`] are called. The former is set by a closure that receives the current
    /// [`IntoIterator`] and returns a context (can consume data from the scope). The second closure
    /// computes the next value from the context. The [`anonymous()`](AnonymouslyIterable::anonymous)
    /// method resembles [`Iterator::scan()`](Iterator::scan) with the exception that the context
    /// may depend on the calling [`IntoIterator`].
    ///
    /// ## Arguments
    ///
    /// * `init_fn`: The closure that receives the current [`IntoIterator`] and produces the initial
    ///   context.
    /// * `next_fn`: The closure that produces the next value from a mutable reference to the context.
    ///
    ///
    /// ## Examples
    ///
    /// First let's create the trivial identity. An [`IntoIterator`] can be consumed:
    ///
    /// ```
    /// use kompost::AnonymouslyIterable;
    /// assert_eq!(
    ///     vec![1,2,3]
    ///         .into_iter()
    ///         .anonymous(
    ///             |it| it.into_iter(),
    ///             |it| it.next()
    ///         )
    ///         .collect::<Vec<_>>(),
    ///     vec![1,2,3]
    /// );
    /// ```
    ///
    /// or borrowed:
    ///
    /// ```
    /// # use kompost::AnonymouslyIterable;
    /// assert_eq!(
    ///     [1, 2, 3]
    ///         .iter()
    ///         .anonymous(|it| it.into_iter().cloned(), |it| it.next())
    ///         .collect::<Vec<_>>(),
    ///     vec![1, 2, 3]
    /// );
    /// ```
    ///
    /// A slightly more complex idea is to collect the iterator first and define
    /// a custom behavior.
    ///
    /// ```
    /// # use kompost::AnonymouslyIterable;
    /// assert_eq!(
    ///     [1, 2, 3]
    ///         .iter()                      // Don't consume
    ///         .anonymous(
    ///             |it| it
    ///                 .into_iter()
    ///                 .rev()               // Revert
    ///                 .copied()            
    ///                 .collect::<Vec<_>>()
    ///                 .into_iter(),        // We need an iterator in next
    ///             |it| it.next().map(|x| x + 4)
    ///         )
    ///         .collect::<Vec<_>>(),
    ///     vec![7, 6, 5]
    /// );
    /// ```
    ///
    /// This enables more complex tasks. For instance, we can transpose a nested iterable
    /// structure ([`IntoIterator`]`<Item =` [`IntoIterator`]`<_>>`) without the need of
    /// writing a single struct or trait! The example is annotated with the inline type
    /// hints as shown by [rust-analyzer lsp](https://rust-analyzer.github.io/):
    ///
    /// ```
    /// # use kompost::AnonymouslyIterable;
    /// let x: Vec<_> = [1, 2, 3, 4]                 // An array in row-major order
    ///     .chunks(2)                               // Nested iterable: Chunks<i32>
    ///     .anonymous(
    ///         |chunks| chunks.map(|row| row.iter()).collect::<Vec<_>>(),
    ///         |context| {
    ///             let transposed = context         // &mut Vec<Iter,i32>
    ///                 .iter_mut()
    ///                 .filter_map(|i| i.next())    // impl Iterator<Item = &i32>
    ///                 .collect::<Vec<_>>();        // Vec<&i32>
    ///                                              // If the iterators over the rows return `None`, transpose is empty
    ///             if transposed.is_empty() {
    ///                 None
    ///             } else {
    ///                 Some(transposed.into_iter())
    ///             }
    ///         },
    ///     )                                        // AnonymousIterator
    ///     .flatten()                               // impl Iterator<Item = &i32>
    ///     .copied()                                // impl Iterator<Item = i32>
    ///     .collect();
    /// assert_eq!(x, [1, 3, 2, 4]);
    /// ```
    ///
    /// This can be conveniently "bundled" in a function—[`transpose()`](crate::transpose)
    /// to be used with the [`composed()`](crate::ComposedIterable::composed) from this crate.
    fn anonymous(
        self,
        init_fn: Init,
        next_fn: Next,
    ) -> AnonymousIterator<Iter, In, Out, Init, Next, Context>;
}

impl<Iter, In, Out, Init, Next, Context> AnonymouslyIterable<Iter, In, Out, Init, Next, Context>
    for Iter
where
    Init: FnOnce(Iter) -> Context,
    Next: Fn(&mut Context) -> Option<Out>,
    Iter: Iterator<Item = In>,
{
    fn anonymous(
        self,
        init_fn: Init,
        next_fn: Next,
    ) -> AnonymousIterator<Iter, In, Out, Init, Next, Context> {
        AnonymousIterator::new(self, init_fn, next_fn)
    }
}
