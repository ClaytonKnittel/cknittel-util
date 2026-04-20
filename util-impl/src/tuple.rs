pub trait TupleTraits {
  type Lhs;
  type Rhs;

  /// The first element of the tuple.
  fn first(self) -> Self::Lhs;

  /// The second element of the tuple.
  fn second(self) -> Self::Rhs;
}

impl<L, R> TupleTraits for (L, R) {
  type Lhs = L;
  type Rhs = R;

  fn first(self) -> L {
    self.0
  }

  fn second(self) -> R {
    self.1
  }
}

impl<'a, L, R> TupleTraits for &'a (L, R) {
  type Lhs = &'a L;
  type Rhs = &'a R;

  fn first(self) -> &'a L {
    &self.0
  }

  fn second(self) -> &'a R {
    &self.1
  }
}

impl<'a, L, R> TupleTraits for &'a mut (L, R) {
  type Lhs = &'a mut L;
  type Rhs = &'a mut R;

  fn first(self) -> &'a mut L {
    &mut self.0
  }

  fn second(self) -> &'a mut R {
    &mut self.1
  }
}
