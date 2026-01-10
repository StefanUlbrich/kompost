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

/// Iterator that with a context and a closures called in the [`ComposedIterator::new`] and
/// [`ComposedIterator::next`] methods. Used in the [`ComposedIterable`] trait.
///
/// [`crate::AnonymousIterator`] could not be used as a constant closure cannot be used within
/// the implementation of [`ComposedIterable`].
pub struct ComposedIterator<IterIn, IterOut, In, Out, Init>
where
    Init: FnOnce(IterIn) -> IterOut,

    IterIn: IntoIterator<Item = In>,
    IterOut: Iterator<Item = Out>,
{
    iter: IterOut,
    _iter: PhantomData<IterIn>,
    _init: PhantomData<Init>,
}

impl<IterIn, IterOut, In, Out, Init> ComposedIterator<IterIn, IterOut, In, Out, Init>
where
    Init: FnOnce(IterIn) -> IterOut,

    IterIn: IntoIterator<Item = In>,
    IterOut: Iterator<Item = Out>,
{
    pub fn new(iter: IterIn, init_fn: Init) -> Self {
        Self {
            iter: init_fn(iter),
            _iter: PhantomData,
            _init: PhantomData,
        }
    }
}

impl<IterIn, IterOut, In, Out, Init> Iterator for ComposedIterator<IterIn, IterOut, In, Out, Init>
where
    Init: FnOnce(IterIn) -> IterOut,

    IterIn: IntoIterator<Item = In>,
    IterOut: Iterator<Item = Out>,
{
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
pub trait ComposedIterable<IterIn, IterOut, In, Out, Init>
where
    Init: FnOnce(IterIn) -> IterOut,

    IterIn: IntoIterator<Item = In>,
    IterOut: Iterator<Item = Out>,
{
    /// Allows defining grouping frequently used [`Iterator`] methods (such as `map` or `scan`)
    /// and reuse them.
    ///
    /// ## Arguments
    ///
    /// * `init_fn`: The closure that receives the current [`IntoIterator`] and produces the initial
    ///   context.
    ///
    /// ## Examples
    ///
    /// ```
    /// use kompost::ComposedIterable;
    ///
    /// fn favorite_pipeline(it: impl IntoIterator<Item = i32>) -> impl Iterator<Item = f64> {
    ///     it.into_iter()
    ///         .skip(5)
    ///         .map(|x| x.pow(2))
    ///         .take_while(|x| *x < 100)
    ///         .map(|x| x as f64)
    /// }
    ///
    /// assert_eq!(
    ///     [1, 2, 3, 4, 5, 6, 7].composed(favorite_pipeline).collect::<Vec<_>>(),
    ///     vec![36.0f64, 49.0]
    /// )
    /// ```
    fn composed(self, init_fs: Init) -> ComposedIterator<IterIn, IterOut, In, Out, Init>;
}

impl<IterIn, IterOut, In, Out, Init> ComposedIterable<IterIn, IterOut, In, Out, Init> for IterIn
where
    Init: FnOnce(IterIn) -> IterOut,

    IterIn: IntoIterator<Item = In>,
    IterOut: Iterator<Item = Out>,
{
    fn composed(self, init_fn: Init) -> ComposedIterator<IterIn, IterOut, In, Out, Init> {
        ComposedIterator::new(self, init_fn)
    }
}
