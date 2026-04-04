use builder::Builder;
use googletest::prelude::*;

#[derive(Builder, Debug)]
struct MyStruct {
  int: i32,
  string: String,
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
    pat!(MyStruct {
      int: eq(&1),
      string: eq("test"),
      v: is_empty()
    })
  );
}
