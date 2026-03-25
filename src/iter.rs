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

#[cfg(test)]
mod tests {
  use std::{error::Error, fmt::Display};

  use googletest::prelude::*;

  use crate::iter::CollectResult;

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
}
