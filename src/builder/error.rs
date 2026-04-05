use std::{
  error::Error,
  fmt::{Debug, Display},
};

#[derive(Clone)]
pub enum BuilderError {
  MissingField { field_name: String },
}

impl BuilderError {
  pub fn missing_field(field_name: impl Into<String>) -> Self {
    let field_name = field_name.into();
    Self::MissingField { field_name }
  }
}

impl Error for BuilderError {}

impl Display for BuilderError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match &self {
      Self::MissingField { field_name } => write!(f, "Missing field `{field_name}`"),
    }
  }
}

impl Debug for BuilderError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self}")
  }
}

pub type BuilderResult<T = ()> = Result<T, BuilderError>;
