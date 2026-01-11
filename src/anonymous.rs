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

/// Trait that adds a method to [`Iterator`] structs (i.e., [`Iterator`]s, [`slice`](std::slice)s, and [`Vec`]s) that
/// allows painlessly create anonynmous iterators for one-time use.
pub trait Anonymous<Iter, In, Out, Init, Next, Context>
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
    /// computes the next value from the context. The [`anonymous()`](Anonymous::anonymous)
    /// method resembles [`Iterator::scan()`](Iterator::scan) with the exception that the context
    /// may depend on the calling [`Iterator`].
    ///
    /// ## Arguments
    ///
    /// * `init_fn`: The closure that receives the current [`Iterator`] and produces the initial
    ///   context.
    /// * `next_fn`: The closure that produces the next value from a mutable reference to the context.
    fn anonymous(
        self,
        init_fn: Init,
        next_fn: Next,
    ) -> AnonymousIterator<Iter, In, Out, Init, Next, Context>;
}

impl<Iter, In, Out, Init, Next, Context> Anonymous<Iter, In, Out, Init, Next, Context> for Iter
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
