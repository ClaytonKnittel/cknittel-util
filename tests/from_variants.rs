#![cfg(feature = "from-variants")]

use from_variants::FromVariants;
use googletest::prelude::*;

#[derive(Debug, FromVariants)]
enum SimpleEnum {
  V1(i32),
  V2(String),
  V3(Vec<u32>),
}

#[gtest]
fn test_simple_enum() {
  expect_that!(SimpleEnum::from(123), pat!(SimpleEnum::V1(eq(&123))));
  expect_that!(
    SimpleEnum::from("123".to_string()),
    pat!(SimpleEnum::V2(eq("123")))
  );
  expect_that!(
    SimpleEnum::from(vec![1, 2, 3]),
    pat!(SimpleEnum::V3(elements_are![eq(&1), eq(&2), eq(&3)]))
  );
}

trait MyTrait {}
impl MyTrait for u64 {}

#[derive(Debug, FromVariants)]
enum GenericEnum<T: MyTrait> {
  V1(T),
  V2(u32),
}

#[gtest]
fn test_generic_enum() {
  type G = GenericEnum<u64>;
  expect_that!(G::from(10u64), pat!(G::V1(eq(&10))));
  expect_that!(G::from(5u32), pat!(G::V2(eq(&5))));
}

#[derive(Debug)]
struct MyStruct<'a, T> {
  val: &'a T,
}

#[derive(Debug, FromVariants)]
enum LifetimeEnum<'a> {
  V1(MyStruct<'a, u32>),
}

#[gtest]
fn test_lifetime_enum() {
  let v = 10u32;
  expect_that!(
    LifetimeEnum::from(MyStruct { val: &v }),
    pat!(LifetimeEnum::V1(pat!(MyStruct { val: eq(&&10) })))
  );
}
