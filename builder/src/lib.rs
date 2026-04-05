mod error;

use proc_macro_error::proc_macro_error;
use proc_macro_util::collect_tokens::TryCollectTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
  Attribute, Data, DeriveInput, Field, GenericArgument, PathArguments, Type, parse_macro_input,
  spanned::Spanned,
};
use util_impl::iter::JoinWith;

use crate::error::{BuilderInternalError, BuilderInternalResult};

enum FieldCategory {
  Default,
  Optional,
  Vec,
}

fn attribute_category(attr: &Attribute) -> Option<FieldCategory> {
  if attr.path().is_ident("optional") {
    Some(FieldCategory::Optional)
  } else if attr.path().is_ident("vec") {
    Some(FieldCategory::Vec)
  } else {
    None
  }
}

fn field_category(field: &Field) -> FieldCategory {
  field
    .attrs
    .iter()
    .find_map(attribute_category)
    .unwrap_or(FieldCategory::Default)
}

fn extract_inner_type<'a>(
  ty: &'a Type,
  expected_type_name: &str,
) -> BuilderInternalResult<&'a Type> {
  let Type::Path(path) = ty else {
    return Err(BuilderInternalError::new(
      format!("Expected field to have type `{expected_type_name}<...>`"),
      ty.span(),
    ));
  };

  let segment = path
    .path
    .segments
    .last()
    .ok_or_else(|| BuilderInternalError::new("Unexpected empty type path", ty.span()))?;

  if segment.ident != expected_type_name {
    return Err(BuilderInternalError::new(
      format!("Expected field to have type `{expected_type_name}<...>`"),
      segment.ident.span(),
    ));
  }

  let PathArguments::AngleBracketed(generics) = &segment.arguments else {
    return Err(BuilderInternalError::new(
      format!("`optional` tag requires explicit generic arguments to `{expected_type_name}`"),
      ty.span(),
    ));
  };

  let inner_generic_argument = generics.args.first().ok_or_else(|| {
    BuilderInternalError::new(
      format!("Unexpected empty generic arguments to `{expected_type_name}`"),
      ty.span(),
    )
  })?;

  if generics.args.len() != 1 {
    return Err(BuilderInternalError::new(
      "Expected only one generic argument",
      segment.arguments.span(),
    ));
  }

  match inner_generic_argument {
    GenericArgument::Type(ty) => Ok(ty),
    _ => Err(BuilderInternalError::new(
      format!("Expected inner generic argument of `{expected_type_name}` to be a type"),
      inner_generic_argument.span(),
    )),
  }
}

fn generate_default_field_member(field: &Field) -> BuilderInternalResult<TokenStream> {
  let ident = field
    .ident
    .as_ref()
    .ok_or_else(|| BuilderInternalError::new("Expect field to have a name", field.span()))?;
  let ty = &field.ty;
  Ok(quote! { #ident: Option<#ty> })
}

fn generate_raw_field_member(field: &Field) -> BuilderInternalResult<TokenStream> {
  let field = Field {
    // Don't copy over any attributes from the original field.
    attrs: Vec::new(),
    ..field.clone()
  };
  Ok(quote! { #field })
}

fn generate_member_for_field(field: &Field) -> BuilderInternalResult<TokenStream> {
  match field_category(field) {
    FieldCategory::Default => generate_default_field_member(field),
    FieldCategory::Optional | FieldCategory::Vec => generate_raw_field_member(field),
  }
}

fn generate_builders_for_singular_field(field: &Field) -> BuilderInternalResult<TokenStream> {
  let ident = field
    .ident
    .as_ref()
    .ok_or_else(|| BuilderInternalError::new("Expect field to have a name", field.span()))?;
  let with = proc_macro2::Ident::new(&format!("with_{}", ident), ident.span());
  let setter = proc_macro2::Ident::new(&format!("set_{}", ident), ident.span());

  let ty = match field_category(field) {
    FieldCategory::Default => &field.ty,
    FieldCategory::Optional => extract_inner_type(&field.ty, "Option")?,
    FieldCategory::Vec => unreachable!(),
  };

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

fn generate_builders_for_list_field(field: &Field) -> BuilderInternalResult<TokenStream> {
  let ident = field
    .ident
    .as_ref()
    .ok_or_else(|| BuilderInternalError::new("Expect field to have a name", field.span()))?;
  let push = proc_macro2::Ident::new(&format!("push_{}", ident), ident.span());
  let add = proc_macro2::Ident::new(&format!("add_{}", ident), ident.span());

  let ty = match field_category(field) {
    FieldCategory::Vec => extract_inner_type(&field.ty, "Vec")?,
    FieldCategory::Default | FieldCategory::Optional => unreachable!(),
  };

  Ok(quote! {
    pub fn #push(&mut self, value: #ty) {
      self.#ident.push(value);
    }
    pub fn #add(mut self, value: #ty) -> Self {
      self.#ident.push(value);
      self
    }
  })
}

fn generate_builders_for_field(field: &Field) -> BuilderInternalResult<TokenStream> {
  match field_category(field) {
    FieldCategory::Default | FieldCategory::Optional => generate_builders_for_singular_field(field),
    FieldCategory::Vec => generate_builders_for_list_field(field),
  }
}

fn generate_build<'a>(
  fields: impl IntoIterator<Item = &'a Field>,
  result_type: &proc_macro2::Ident,
) -> BuilderInternalResult<TokenStream> {
  let field_initializers = fields
    .into_iter()
    .map(|field| {
      let ident = field
        .ident
        .as_ref()
        .expect("Already asserted that field has ident");
      let field_name_str = ident.to_string();

      Ok(match field_category(field) {
        FieldCategory::Default => quote! {
          #ident: self.#ident.ok_or_else(|| {
            ::cknittel_util::builder::error::BuilderError::missing_field(#field_name_str)
          })?,
        },
        FieldCategory::Optional | FieldCategory::Vec => quote! {
          #ident: self.#ident,
        },
      })
    })
    .try_collect_tokens()?;

  Ok(quote! {
    pub fn build(self) -> ::cknittel_util::builder::error::BuilderResult<#result_type> {
      Ok(#result_type {
        #field_initializers
      })
    }
  })
}

fn build_builder_impl(input: DeriveInput) -> BuilderInternalResult<TokenStream> {
  let Data::Struct(data) = input.data else {
    return Err(BuilderInternalError::new(
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
#[proc_macro_derive(Builder, attributes(optional, vec))]
/// Constructs a builder class.
pub fn derive_builder(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(tokens as DeriveInput);

  match build_builder_impl(input) {
    Ok(tokens) => tokens.into(),
    Err(err) => err.abort(),
  }
}
