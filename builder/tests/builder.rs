use builder::Builder;
use googletest::prelude::*;

#[derive(Builder)]
struct MyStruct {
  int: i32,
  string: String,
  v: Vec<i32>,
}

#[gtest]
fn test_build() {
  // let mut builder = MyStructBuilder::default();
}
