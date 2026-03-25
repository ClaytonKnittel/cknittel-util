use std::{
  error::Error,
  fmt::{Debug, Display},
};

use proc_macro::Span;
use proc_macro_error::abort;

#[derive(Clone)]
pub struct EnumExtractorError {
  message: String,
  span: Span,
}

impl EnumExtractorError {
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

impl Error for EnumExtractorError {}

impl Display for EnumExtractorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Debug for EnumExtractorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type EnumExtractorResult<T = ()> = Result<T, EnumExtractorError>;
