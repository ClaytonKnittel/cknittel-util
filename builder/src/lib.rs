mod error;

use proc_macro_error::proc_macro_error;
use proc_macro2::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

use crate::error::{BuilderError, BuilderResult};

fn test(input: DeriveInput) -> BuilderResult<TokenStream> {
  let Data::Struct(data) = input.data else {
    return Err(BuilderError::new(
      "Can only derive `Builder` on a struct",
      input.ident.span(),
    ));
  };

  for field in &data.fields {}

  Ok(TokenStream::new())
}

#[proc_macro_error]
#[proc_macro_derive(Builder)]
/// Constructs an LR(1) parser based on the definition provided.
pub fn derive_builder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  match test(input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.abort(),
  }
}
