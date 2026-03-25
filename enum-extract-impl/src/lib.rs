mod error;

use proc_macro_error::proc_macro_error;
use proc_macro_util::collect_tokens::TryCollectTokens;
use quote::quote;
use syn::{spanned::Spanned, DataEnum, FieldsUnnamed};

use crate::error::{EnumExtractorError, EnumExtractorResult};

fn generate_accessor(fields: &FieldsUnnamed) -> EnumExtractorResult<proc_macro2::TokenStream> {
  if fields.unnamed.len() == 1 {
    let field = &fields.unnamed[0];
  } else {
    todo!();
  }

  Ok(quote! {})
}

fn generate_variant_accessors(data: &DataEnum) -> EnumExtractorResult<proc_macro2::TokenStream> {
  data
    .variants
    .iter()
    .map(|variant| match &variant.fields {
      syn::Fields::Unit => Ok(proc_macro2::TokenStream::new()),
      syn::Fields::Unnamed(fields) => {
        debug_assert!(!fields.unnamed.is_empty());
        generate_accessor(fields)
      }
      syn::Fields::Named(_) => Err(EnumExtractorError::new(
        "Can't use named fields with EnumExtract",
        variant.span(),
      )),
    })
    .try_collect_tokens()
}

fn build_extractor(ast: syn::DeriveInput) -> EnumExtractorResult<proc_macro2::TokenStream> {
  let data = match ast.data {
    syn::Data::Enum(data) => data,
    _ => {
      return Err(EnumExtractorError::new(
        "\"EnumExtract\" may only be used on enums",
        ast.ident.span(),
      ))
    }
  };

  let variants = generate_variant_accessors(&data)?;

  Ok(quote! {
    impl $enum_name {
      #variants
    }
  })
}

#[proc_macro_error]
#[proc_macro_derive(EnumExtract)]
pub fn extract(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);

  match build_extractor(ast) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.abort(),
  }
}
