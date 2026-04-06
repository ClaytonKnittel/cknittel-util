#![cfg(feature = "from-variants")]

use from_variants::FromVariants;
use googletest::prelude::*;

#[derive(Debug, FromVariants)]
enum SimpleEnum {
  V1(i32),
  V2(String),
}

#[gtest]
fn test_simple_enum() {
  expect_that!(SimpleEnum::from(123), pat!(SimpleEnum::V1(eq(&123))));
  expect_that!(
    SimpleEnum::from("123".to_string()),
    pat!(SimpleEnum::V2(eq("123")))
  );
}
