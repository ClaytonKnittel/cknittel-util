#![cfg(feature = "builder")]

use builder::Builder;
use cknittel_util::builder::error::BuilderError;
use googletest::prelude::*;

#[derive(Builder, Debug)]
struct MyStruct {
  int: i32,
  string: String,
  optional_int: Option<i32>,
  v: Vec<i32>,
}

#[gtest]
fn test_build() {
  let mut builder = MyStructBuilder::default();
  builder.set_int(1);

  let builder = builder.with_string("test".to_owned());
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(MyStruct {
      int: eq(&1),
      string: eq("test"),
      optional_int: none(),
      v: is_empty()
    }))
  );
}

#[gtest]
fn test_missing_field() {
  let mut builder = MyStructBuilder::default();
  builder.set_int(1);

  let res = builder.build();

  expect_that!(
    res,
    err(pat!(BuilderError::MissingField {
      field_name: eq("string")
    }))
  );
}
