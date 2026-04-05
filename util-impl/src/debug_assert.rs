#[macro_export]
macro_rules! debug_assert_le {
  ($lhs:tt, $rhs:tt) => {
    if cfg!(debug_assertions) {
      let lhs = $lhs;
      let rhs = $rhs;
      ::core::assert!(lhs <= rhs, "Expected {lhs} <= {rhs}");
    }
  };
}

#[macro_export]
macro_rules! debug_assert_lt {
  ($lhs:tt, $rhs:tt) => {
    if cfg!(debug_assertions) {
      let lhs = $lhs;
      let rhs = $rhs;
      ::core::assert!(lhs < rhs, "Expected {lhs} < {rhs}");
    }
  };
}

#[macro_export]
macro_rules! debug_assert_ge {
  ($lhs:tt, $rhs:tt) => {
    if cfg!(debug_assertions) {
      let lhs = $lhs;
      let rhs = $rhs;
      ::core::assert!(lhs >= rhs, "Expected {lhs} >= {rhs}");
    }
  };
}

#[macro_export]
macro_rules! debug_assert_gt {
  ($lhs:tt, $rhs:tt) => {
    if cfg!(debug_assertions) {
      let lhs = $lhs;
      let rhs = $rhs;
      ::core::assert!(lhs > rhs, "Expected {lhs} > {rhs}");
    }
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_debug_assertions() {
    debug_assert_le!(0, 1);
    debug_assert_le!(1, 1);
    debug_assert_lt!(0, 1);
    debug_assert_ge!(1, 0);
    debug_assert_ge!(1, 1);
    debug_assert_gt!(1, 0);
  }
}
