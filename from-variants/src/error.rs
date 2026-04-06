use std::{
  error::Error,
  fmt::{Debug, Display},
};

use proc_macro_error::abort;
use proc_macro2::Span;

#[derive(Clone)]
pub struct FromVariantsInternalError {
  message: String,
  span: Span,
}

impl FromVariantsInternalError {
  pub fn new(message: impl Into<String>, span: Span) -> Self {
    Self {
      message: message.into(),
      span,
    }
  }

  pub fn abort(&self) -> ! {
    abort!(self.span, self.message);
  }
}

impl Error for FromVariantsInternalError {}

impl Display for FromVariantsInternalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for FromVariantsInternalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type FromVariantsInternalResult<T = ()> = Result<T, FromVariantsInternalError>;
