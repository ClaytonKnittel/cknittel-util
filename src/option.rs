pub trait OptionUtil {
  type Inner;

  /// Erases the inner value from `Some(..)`, yielding an `Option<()>`.
  fn erase_some(self) -> Option<()>;
}

impl<T> OptionUtil for Option<T> {
  type Inner = T;

  fn erase_some(self) -> Option<()> {
    self.map(|_| ())
  }
}
