use std::{cell::UnsafeCell, fmt::Debug, iter::Peekable, ops::Deref};

use crate::option::OptionUtil;

pub trait IntoPeekableStream<I: Iterator> {
  fn peekable_stream(self) -> PeekableStream<I>;
}

impl<I: Iterator> IntoPeekableStream<I> for I {
  fn peekable_stream(self) -> PeekableStream<I> {
    PeekableStream::new(self)
  }
}

pub struct PeekableStream<I: Iterator> {
  stream: UnsafeCell<Peekable<I>>,
}

impl<I> PeekableStream<I>
where
  I: Iterator,
{
  pub fn new(iter: impl Into<I>) -> Self {
    Self {
      stream: UnsafeCell::new(iter.into().peekable()),
    }
  }

  fn inner_mut(&mut self) -> &mut Peekable<I> {
    unsafe { &mut *self.stream.get() }
  }

  #[allow(clippy::mut_from_ref)]
  unsafe fn inner_mut_unsafe(&self) -> &mut Peekable<I> {
    unsafe { &mut *self.stream.get() }
  }

  unsafe fn peek_without_modification(&self) -> Option<&I::Item> {
    unsafe { self.inner_mut_unsafe() }.peek()
  }

  pub fn peek(&mut self) -> Option<ItemProxy<'_, I>> {
    self
      .inner_mut()
      .peek()
      .erase_some()
      .map(|_| ItemProxy::new(self))
  }
}

impl<I: Iterator> Iterator for PeekableStream<I> {
  type Item = I::Item;

  fn next(&mut self) -> Option<Self::Item> {
    self.inner_mut().next()
  }
}

/// A type returned by `peek` from `PeekableStream`s when the peeked value is
/// present. Can be consumed with `take()` to advance the stream, returning the
/// peeked value without needing `unwrap()`.
pub struct ItemProxy<'a, I: Iterator> {
  iter: &'a mut PeekableStream<I>,
}

impl<'a, I> ItemProxy<'a, I>
where
  I: Iterator,
{
  fn new(iter: &'a mut PeekableStream<I>) -> Self {
    Self { iter }
  }

  /// Advances the symbol stream and returns the peeked symbol.
  pub fn take(self) -> I::Item {
    let next = self.iter.next();
    debug_assert!(next.is_some());
    unsafe { next.unwrap_unchecked() }
  }
}

impl<'a, I: Iterator> Deref for ItemProxy<'a, I> {
  type Target = I::Item;

  fn deref(&self) -> &I::Item {
    unsafe { self.iter.peek_without_modification().unwrap_unchecked() }
  }
}

impl<'a, I: Iterator> Debug for ItemProxy<'a, I>
where
  I::Item: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.deref())
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::peekable_stream::IntoPeekableStream;

  #[gtest]
  fn test_non_cloneable() {
    #[derive(Debug, PartialEq, Eq)]
    struct Char(char);
    let chars = ['a', 'b', 'c', 'd', 'e'].map(Char);

    let mut stream = chars.into_iter().peekable_stream();
    let peeked = stream.peek();
    assert_that!(peeked.as_deref(), some(eq(&Char('a'))));
    expect_eq!(peeked.unwrap().take(), Char('a'));

    expect_that!(stream.peek().as_deref(), some(eq(&Char('b'))));

    expect_that!(stream.next(), some(eq(&Char('b'))));
    expect_that!(stream.next(), some(eq(&Char('c'))));
    expect_that!(stream.next(), some(eq(&Char('d'))));
    expect_that!(stream.next(), some(eq(&Char('e'))));

    expect_that!(stream.peek(), none());
  }
}
