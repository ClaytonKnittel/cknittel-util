use std::{
  error::Error,
  fmt::{Debug, Display},
};

use proc_macro_error::abort;
use proc_macro2::Span;

#[derive(Clone)]
pub struct BuilderError {
  message: String,
  span: Span,
}

impl BuilderError {
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
    abort!(self.span, self.message)
  }
}

impl Error for BuilderError {}

impl Display for BuilderError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for BuilderError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type BuilderResult<T = ()> = Result<T, BuilderError>;
