use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, Generics};
use util_impl::iter::JoinWith;

use crate::collect_tokens::CollectTokens;

pub trait StripTraitBounds {
  fn strip_trait_bounds(&self) -> TokenStream;
}

impl StripTraitBounds for Generics {
  fn strip_trait_bounds(&self) -> TokenStream {
    if self.params.is_empty() {
      return TokenStream::new();
    }

    std::iter::once(quote! { < })
      .chain(
        self
          .params
          .iter()
          .map(|param| match param {
            GenericParam::Lifetime(lifetime) => quote! { #lifetime },
            GenericParam::Type(type_param) => {
              let ident = &type_param.ident;
              quote! { #ident }
            }
            GenericParam::Const(const_param) => {
              let ident = &const_param.ident;
              quote! { #ident }
            }
          })
          .join_with(|| quote! { , }),
      )
      .chain(std::iter::once(quote! { > }))
      .collect_tokens()
  }
}
