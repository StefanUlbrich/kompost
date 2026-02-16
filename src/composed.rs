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

//! Iterator compositions needs its own implementation separate from anonymous iterators
//! as each closure has its own type and hence, a constant closure cannot be used within the
//! [Anonymous](crate::Anonymous) trait.

pub trait Composed<IterOut, Func>: Sized
where
    IterOut: Iterator,
    Func: FnOnce(Self) -> IterOut,
{
    /// Allows defining grouping frequently used [`Iterator`] methods (such as `map` or `scan`)
    /// and reuse them.
    ///
    /// ## Arguments
    ///
    /// * `init_fn`: The closure that receives the current [`IntoIterator`] and produces the initial
    ///   context.
    fn composed(self, init_fs: Func) -> IterOut;
}

impl<Iter, IterOut, Func> Composed<IterOut, Func> for Iter
where
    IterOut: Iterator,
    Func: FnOnce(Iter) -> IterOut,
{
    fn composed(self, init_fs: Func) -> IterOut {
        init_fs(self)
    }
}
