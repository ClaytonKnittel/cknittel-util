#![cfg(feature = "builder")]

use cknittel_util::builder::{error::BuilderError, Builder};
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

#[derive(Builder, Debug)]
struct Generic<T> {
  field: T,
  #[vec]
  vec: Vec<T>,
}

#[gtest]
fn test_build_generic() {
  let builder = GenericBuilder::default();
  let mut builder = builder.with_field("test".to_string());
  builder.push_vec("some_str".to_string());
  let builder = builder.add_vec("other_str".to_string());
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(Generic {
      field: "test",
      vec: elements_are![eq("some_str"), eq("other_str")],
    }))
  );
}

#[derive(Builder, Debug)]
struct GenericWithBounds<T: Default, U>
where
  U: Default,
{
  field1: T,
  field2: U,
}

#[gtest]
fn test_build_generic_with_bounds() {
  let mut builder = GenericWithBoundsBuilder::default();
  builder.set_field1(1.2);
  builder.set_field2(100);
  let res = builder.build();

  expect_that!(
    res,
    ok(pat!(GenericWithBounds {
      field1: eq(&1.2),
      field2: eq(&100)
    }))
  );
}
