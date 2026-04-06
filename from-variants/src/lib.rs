mod error;

use proc_macro_error::proc_macro_error;
use proc_macro_util::collect_tokens::TryCollectTokens;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Variant, parse_macro_input, spanned::Spanned};

use crate::error::{FromVariantsInternalError, FromVariantsInternalResult};

type TokenStreamResult = FromVariantsInternalResult<TokenStream>;

fn impl_from_variant(variant: &Variant, enum_ident: &Ident) -> TokenStreamResult {
  let variant_name = &variant.ident;

  let fields = match &variant.fields {
    Fields::Unnamed(unnamed) => unnamed,
    Fields::Unit => return Ok(TokenStream::new()),
    Fields::Named(named_fields) => {
      return Err(FromVariantsInternalError::new(
        "FromVariants requires all enum variants to either have a single unnamed data member, or be empty",
        named_fields.span(),
      ));
    }
  };

  let unnamed = &fields.unnamed;

  if unnamed.len() != 1 {
    return Err(FromVariantsInternalError::new(
      "FromVariants requires all enum variants to either have a single unnamed data member, or be empty",
      unnamed.span(),
    ));
  }

  let data = unnamed.first().expect("Already checked length == 1");
  let ty = &data.ty;

  Ok(quote! {
    impl From<#ty> for #enum_ident {
      fn from(value: #ty) -> Self {
        Self::#variant_name(value)
      }
    }
  })
}

fn build_from_variants_impl(input: DeriveInput) -> TokenStreamResult {
  let Data::Enum(data) = input.data else {
    return Err(FromVariantsInternalError::new(
      "Can only derive `FromVariants` on enums",
      input.ident.span(),
    ));
  };

  let input_ident = &input.ident;

  let variant_impls = data
    .variants
    .iter()
    .map(|variant| impl_from_variant(variant, input_ident))
    .try_collect_tokens()?;

  Ok(quote! {
    #variant_impls
  })
}

#[proc_macro_error]
#[proc_macro_derive(FromVariants)]
/// Constructs a builder class.
pub fn derive_from_variants(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  match build_from_variants_impl(input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.abort(),
  }
}
