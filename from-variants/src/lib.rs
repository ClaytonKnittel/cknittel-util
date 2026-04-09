mod error;

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::proc_macro_error;
use proc_macro_util::collect_tokens::{CollectTokens, TryCollectTokens};
use quote::quote;
use syn::{
  parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, GenericParam, Generics, Variant,
};
use util_impl::iter::JoinWith;

use crate::error::{FromVariantsInternalError, FromVariantsInternalResult};

type TokenStreamResult = FromVariantsInternalResult<TokenStream>;

fn generic_args(generics: &Generics) -> TokenStream {
  generics
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
    .join_with(|| quote! { , })
    .collect_tokens()
}

fn impl_from_variant(
  variant: &Variant,
  enum_ident: &Ident,
  generics: &Generics,
) -> TokenStreamResult {
  let variant_name = &variant.ident;
  let generic_args = generic_args(generics);

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
    impl #generics From<#ty> for #enum_ident<#generic_args> {
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
  let generics = &input.generics;

  let variant_impls = data
    .variants
    .iter()
    .map(|variant| impl_from_variant(variant, input_ident, generics))
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
