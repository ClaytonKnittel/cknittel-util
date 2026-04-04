mod error;

use cknittel_util::{
  iter::JoinWith,
  proc_macro_util::collect_tokens::{CollectTokens, TryCollectTokens},
};
use proc_macro_error::proc_macro_error;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, parse_macro_input, spanned::Spanned};

use crate::error::{BuilderError, BuilderResult};

fn generate_default_field_member(field: &Field) -> BuilderResult<TokenStream> {
  let ident = field
    .ident
    .as_ref()
    .ok_or_else(|| BuilderError::new("Expect field to have a name", field.span()))?;
  let ty = &field.ty;
  Ok(quote! { #ident: Option<#ty> })
}

fn generate_member_for_field(field: &Field) -> BuilderResult<TokenStream> {
  generate_default_field_member(field)
}

fn generate_default_builders(field: &Field) -> BuilderResult<TokenStream> {
  let ident = field
    .ident
    .as_ref()
    .ok_or_else(|| BuilderError::new("Expect field to have a name", field.span()))?;
  let with = proc_macro2::Ident::new(&format!("with_{}", ident), ident.span());
  let setter = proc_macro2::Ident::new(&format!("set_{}", ident), ident.span());
  let ty = &field.ty;

  Ok(quote! {
    pub fn #setter(&mut self, value: #ty) {
      self.#ident.replace(value);
    }
    pub fn #with(mut self, value: #ty) -> Self {
      self.#ident.replace(value);
      self
    }
  })
}

fn generate_builders_for_field(field: &Field) -> BuilderResult<TokenStream> {
  generate_default_builders(field)
  // match &field.ty {
  //   Type::Path(path) => {}
  //   _ => {}
  // }
}

fn generate_build<'a>(
  fields: impl IntoIterator<Item = &'a Field>,
  result_type: &proc_macro2::Ident,
) -> BuilderResult<TokenStream> {
  let field_initializers = fields
    .into_iter()
    .map(|field| {
      let ident = field
        .ident
        .as_ref()
        .expect("Already asserted that field has ident");
      quote! {
        #ident: self.#ident.unwrap()
      }
    })
    .join_with(|| quote! { , })
    .collect_tokens();

  Ok(quote! {
    pub fn build(self) -> #result_type {
      #result_type {
        #field_initializers
      }
    }
  })
}

fn test(input: DeriveInput) -> BuilderResult<TokenStream> {
  let Data::Struct(data) = input.data else {
    return Err(BuilderError::new(
      "Can only derive `Builder` on a struct",
      input.ident.span(),
    ));
  };

  let builder_ident =
    proc_macro2::Ident::new(&format!("{}Builder", input.ident), input.ident.span());

  // Copy fields from the original struct.
  let fields = data
    .fields
    .iter()
    .map(generate_member_for_field)
    .join_with(|| Ok(quote! { , }))
    .try_collect_tokens()?;

  let field_builders = data
    .fields
    .iter()
    .map(generate_builders_for_field)
    .try_collect_tokens()?;

  let builder = generate_build(data.fields.iter(), &input.ident)?;

  Ok(quote! {
    #[derive(Default)]
    struct #builder_ident {
      #fields
    }
    impl #builder_ident {
      #field_builders
      #builder
    }
  })
}

#[proc_macro_error]
#[proc_macro_derive(Builder)]
/// Constructs a builder class.
pub fn derive_builder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  match test(input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.abort(),
  }
}
