use num_traits::PrimInt;

pub trait IterOnes {
  /// Given an integer, returns an iterator over the bit indices with ones.
  fn iter_ones(self) -> impl Iterator<Item = u32>;
}

impl<I: PrimInt> IterOnes for I {
  fn iter_ones(self) -> impl Iterator<Item = u32> {
    let if_ne_zero = |value: I| (value != I::zero()).then_some(value);
    std::iter::successors(if_ne_zero(self), move |&value| {
      let value = value & (value - I::one());
      if_ne_zero(value)
    })
    .map(|mask| mask.trailing_zeros())
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;
  use itertools::Itertools;

  use crate::iter_ones::IterOnes;

  #[gtest]
  fn iter_ones() {
    expect_that!(1.iter_ones().collect_vec(), elements_are![&0]);
    expect_that!(0x01_00.iter_ones().collect_vec(), elements_are![&8]);

    expect_that!(
      0xff_ff.iter_ones().collect_vec(),
      elements_are![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15]
    );

    expect_that!(4i8.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4i16.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4i32.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4i64.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4i128.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4isize.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4u8.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4u16.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4u32.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4u64.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4u128.iter_ones().collect_vec(), elements_are![&2]);
    expect_that!(4usize.iter_ones().collect_vec(), elements_are![&2]);
  }
}
