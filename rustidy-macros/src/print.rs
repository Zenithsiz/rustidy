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
	let (print, print_non_ws) = match &attrs.data {
		darling::ast::Data::Enum(variants) => {
			let (print, print_non_ws) = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					let print = quote! { Self::#variant_ident(ref value) => print::Print::print(value, f), };
					let print_non_ws = quote! { Self::#variant_ident(ref value) => print::Print::print_non_ws(value, f), };

					(print, print_non_ws)
				})
				.collect::<(Vec<_>, Vec<_>)>();

			let print = quote! { match *self { #( #print )* } };
			let print_non_ws = quote! { match *self { #( #print_non_ws )* } };

			(print, print_non_ws)
		},

		darling::ast::Data::Struct(fields) => {
			let (print, print_non_ws) = fields
				.fields
				.iter()
				.enumerate()
				.map(|(field_idx, field)| {
					let field_ident = util::field_member_access(field_idx, field);
					let print = quote! { print::Print::print(&self.#field_ident, f); };
					let print_non_ws = quote! { print::Print::print_non_ws(&self.#field_ident, f); };

					(print, print_non_ws)
				})
				.collect::<(Vec<_>, Vec<_>)>();

			let print = quote! { #( #print )* };
			let print_non_ws = quote! { #( #print_non_ws )* };

			(print, print_non_ws)
		},
	};

	let impl_generics = util::with_bounds(&attrs, |ty| parse_quote! { #ty: print::Print });
	let (impl_generics, ty_generics, fmt_where_clause) = impl_generics.split_for_impl();
	let output = quote! {
		impl #impl_generics print::Print for #item_ident #ty_generics #fmt_where_clause {
			#[coverage(on)]
			fn print(&self, f: &mut print::PrintFmt) {
				#print
			}

			#[coverage(on)]
			fn print_non_ws(&self, f: &mut print::PrintFmt) {
				#print_non_ws
			}
		}
	};

	Ok(output.into())
}
