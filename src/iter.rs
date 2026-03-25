pub trait CollectResult<T, E> {
  fn collect_result<C>(self) -> Result<C, E>
  where
    C: FromIterator<T>;

  fn collect_result_vec(self) -> Result<Vec<T>, E>;
}

impl<T, E, I> CollectResult<T, E> for I
where
  I: Iterator<Item = Result<T, E>>,
{
  fn collect_result<C>(self) -> Result<C, E>
  where
    C: FromIterator<T>,
  {
    self.collect()
  }

  fn collect_result_vec(self) -> Result<Vec<T>, E> {
    self.collect_result()
  }
}

pub trait JoinWith<T> {
  fn join_with<F>(self, f: F) -> impl Iterator<Item = T>
  where
    F: FnMut() -> T;
}

struct JoinWithImpl<I, T, F> {
  iter: I,
  next: Option<T>,
  take_from_sep: bool,
  sep_generator: F,
}

impl<I, T, F> Iterator for JoinWithImpl<I, T, F>
where
  I: Iterator<Item = T>,
  F: FnMut() -> T,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.take_from_sep {
      self.take_from_sep = false;
      Some((self.sep_generator)())
    } else {
      let result = self.next.take()?;
      self.next = self.iter.next();
      self.take_from_sep = self.next.is_some();
      Some(result)
    }
  }
}

impl<T, I> JoinWith<T> for I
where
  I: Iterator<Item = T>,
{
  fn join_with<F>(mut self, f: F) -> impl Iterator<Item = T>
  where
    F: FnMut() -> T,
  {
    let next = self.next();
    JoinWithImpl {
      iter: self,
      next,
      take_from_sep: false,
      sep_generator: f,
    }
  }
}

#[cfg(test)]
mod tests {
  use std::{error::Error, fmt::Display};

  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::iter::{CollectResult, JoinWith};

  #[derive(Debug)]
  struct TestError;
  impl Display for TestError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      Ok(())
    }
  }
  impl Error for TestError {}

  #[gtest]
  fn test_collect_result() {
    let iter = [Ok::<_, TestError>(1), Ok(2), Ok(3)].into_iter();

    expect_that!(
      iter.collect_result::<Vec<_>>(),
      ok(elements_are![&1, &2, &3])
    );
  }

  #[gtest]
  fn test_collect_result_err() {
    let iter = [Ok(1), Ok(2), Err(TestError)].into_iter();

    expect_that!(iter.collect_result::<Vec<_>>(), err(anything()));
  }

  #[gtest]
  fn test_collect_result_vec() {
    let iter = [Ok::<_, TestError>(1), Ok(2), Ok(3)].into_iter();

    expect_that!(iter.collect_result_vec(), ok(elements_are![&1, &2, &3]));
  }

  #[gtest]
  fn test_collect_result_vec_err() {
    let iter = [Ok(1), Ok(2), Err(TestError)].into_iter();

    expect_that!(iter.collect_result_vec(), err(anything()));
  }

  #[gtest]
  fn test_join_with() {
    let iter = [1, 2, 3, 4, 5].into_iter();
    let mut v = 10;

    expect_that!(
      iter
        .join_with(|| {
          let res = v;
          v += 1;
          res
        })
        .collect_vec(),
      elements_are![&1, &10, &2, &11, &3, &12, &4, &13, &5]
    );
  }

  #[gtest]
  fn test_join_with_empty() {
    expect_that!([].into_iter().join_with(|| 10).collect_vec(), is_empty());
  }
}
