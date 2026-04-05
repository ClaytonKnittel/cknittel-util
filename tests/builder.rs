#![cfg(feature = "builder")]

use cknittel_util::builder::{Builder, error::BuilderError};
use googletest::prelude::*;

#[derive(Builder, Debug)]
struct SimpleStruct {
  int: i32,
  string: String,
}

#[gtest]
fn test_build_simple() {
  let mut builder = SimpleStructBuilder::default();
  builder.set_int(1);

  let builder = builder.with_string("test".to_owned());
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(SimpleStruct {
      int: eq(&1),
      string: eq("test"),
    }))
  );
}

#[gtest]
fn test_missing_field() {
  let mut builder = SimpleStructBuilder::default();
  builder.set_int(1);

  let res = builder.build();

  expect_that!(
    res,
    err(pat!(BuilderError::MissingField {
      field_name: eq("string")
    }))
  );
}

#[derive(Builder, Debug)]
struct OptionalField {
  #[optional]
  optional_string: Option<String>,
}

#[gtest]
fn test_build_optional() {
  let mut builder = OptionalFieldBuilder::default();
  builder.set_optional_string("some_str".to_string());
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(OptionalField {
      optional_string: some(eq("some_str")),
    }))
  );
}

#[gtest]
fn test_build_missing_optional() {
  let builder = OptionalFieldBuilder::default();
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(OptionalField {
      optional_string: none(),
    }))
  );
}

#[derive(Builder, Debug)]
struct VecField {
  #[vec]
  strings: Vec<String>,
}

#[gtest]
fn test_build_vec() {
  let mut builder = VecFieldBuilder::default();
  builder.push_strings("some_str".to_string());
  let builder = builder.add_strings("other_str".to_string());
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(VecField {
      strings: elements_are![eq("some_str"), eq("other_str")],
    }))
  );
}

#[gtest]
fn test_build_empty_vec() {
  let builder = VecFieldBuilder::default();
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(VecField {
      strings: is_empty(),
    }))
  );
}
