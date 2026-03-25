use proc_macro2::TokenStream;

pub trait CollectTokens {
  fn collect_tokens(self) -> proc_macro2::TokenStream;
}

impl<T, I> CollectTokens for T
where
  T: IntoIterator<Item = I>,
  I: Into<proc_macro2::TokenStream>,
{
  fn collect_tokens(self) -> proc_macro2::TokenStream {
    self
      .into_iter()
      .fold(proc_macro2::TokenStream::new(), |mut tokens, item| {
        tokens.extend(item.into());
        tokens
      })
  }
}

pub trait TryCollectTokens<E> {
  fn try_collect_tokens(self) -> Result<TokenStream, E>;
}

impl<T, I, E> TryCollectTokens<E> for T
where
  T: IntoIterator<Item = Result<I, E>>,
  I: Into<TokenStream>,
{
  fn try_collect_tokens(self) -> Result<TokenStream, E> {
    self
      .into_iter()
      .try_fold(proc_macro2::TokenStream::new(), |mut tokens, item| {
        tokens.extend(item?.into());
        Ok(tokens)
      })
  }
}
