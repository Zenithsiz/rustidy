//! `derive(Print)`

// Imports
use {
	crate::util,
	app_error::{AppError, Context},
	darling::FromDeriveInput,
	quote::quote,
	syn::parse_quote,
};

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(print))]
struct VariantFieldAttrs {
	#[as_ref]
	_ident: Option<syn::Ident>,
	#[as_ref]
	ty:     syn::Type,
}

#[derive(Debug, darling::FromVariant, derive_more::AsRef)]
#[darling(attributes(print))]
struct VariantAttrs {
	ident:  syn::Ident,
	fields: darling::ast::Fields<VariantFieldAttrs>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(print))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,
}

#[derive(Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(print))]
struct Attrs {
	ident:    syn::Ident,
	generics: syn::Generics,
	data:     darling::ast::Data<VariantAttrs, FieldAttrs>,
}

pub fn derive(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> {
	let input = syn::parse::<syn::DeriveInput>(input)
		.context("Unable to parse input")?;

	let attrs = Attrs::from_derive_input(&input)
		.context("Unable to parse attributes")?;
	let item_ident = &attrs.ident;

	// Parse body, parsable impl and error enum (with it's impls)
	let fmt_body = match &attrs.data {
		darling::ast::Data::Enum(variants) => {
			let fmt_variant = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					quote! { Self::#variant_ident(ref value) => rustidy_print::Print::print(value, f), }
				})
				.collect::<Vec<_>>();

			let body = quote! {
				match *self {
					#( #fmt_variant )*
				}
			};

			body
		},

		darling::ast::Data::Struct(fields) => {
			let fmt_fields = fields
				.fields
				.iter()
				.enumerate()
				.map(|(field_idx, field)| {
					let field_ident = util::field_member_access(field_idx, field);
					quote! { rustidy_print::Print::print(&self.#field_ident, f); }
				})
				.collect::<Vec<_>>();

			let body = quote! {
				#( #fmt_fields )*
			};

			body
		},
	};

	let impl_generics = util::with_bounds(&attrs, |ty| parse_quote! { #ty: rustidy_print::Print });
	let (impl_generics, ty_generics, fmt_where_clause) = impl_generics.split_for_impl();
	let output = quote! {
		impl #impl_generics rustidy_print::Print for #item_ident #ty_generics #fmt_where_clause {
			#[coverage(on)]
			fn print(&self, f: &mut rustidy_print::PrintFmt) {
				#fmt_body
			}
		}
	};

	Ok(output.into())
}
