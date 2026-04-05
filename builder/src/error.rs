use std::{
  error::Error,
  fmt::{Debug, Display},
};

use proc_macro_error::abort;
use proc_macro2::Span;

#[derive(Clone)]
pub struct BuilderInternalError {
  message: String,
  span: Span,
}

impl BuilderInternalError {
  pub fn new(message: impl Into<String>, span: Span) -> Self {
    Self {
      message: message.into(),
      span,
    }
  }

  pub fn from_foreign_error(error: impl Error, span: Span) -> Self {
    Self::new(format!("{error}"), span)
  }

  pub fn abort(&self) -> ! {
    abort!(self.span, self.message);
    unreachable!()
  }
}

impl Error for BuilderInternalError {}

impl Display for BuilderInternalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for BuilderInternalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type BuilderInternalResult<T = ()> = Result<T, BuilderInternalError>;
