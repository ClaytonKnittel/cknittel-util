mod error;

use proc_macro::TokenStream;

use crate::error::EnumExtractorResult;

fn build_extractor(tokens: TokenStream) -> EnumExtractorResult<TokenStream> {
  Ok(TokenStream::new())
}

#[proc_macro_derive(Extract)]
pub fn extract(tokens: TokenStream) -> TokenStream {
  match build_extractor(tokens) {
    Ok(tokens) => tokens,
    Err(err) => err.abort(),
  }
}
