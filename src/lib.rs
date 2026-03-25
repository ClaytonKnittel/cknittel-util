pub use enum_extract_impl::Extract as EnumExtract;

#[derive(EnumExtract)]
enum Test {
  A(i32),
  B(i32),
  C(u32),
}
