pub trait EraseOk<E> {
  /// Erases the value of a result, replacing it with `()`. Equivalent to
  /// calling `.map(|_| ())` on the result.
  fn erase_ok(self) -> Result<(), E>;
}

impl<T, E> EraseOk<E> for Result<T, E> {
  fn erase_ok(self) -> Result<(), E> {
    self.map(|_| ())
  }
}

pub trait CloneErr<T, E> {
  /// Clones an error through a reference to a result. This is like
  /// `Option::as_ref`.
  fn clone_err(&self) -> Result<&T, E>;
}

impl<T, E: Clone> CloneErr<T, E> for Result<T, E> {
  fn clone_err(&self) -> Result<&T, E> {
    match self {
      Ok(value) => Ok(value),
      Err(err) => Err(err.clone()),
    }
  }
}
