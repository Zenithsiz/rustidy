//! `derive(Formattable)`

// Imports
use {
	crate::util,
	app_error::{AppError, Context},
	darling::FromDeriveInput,
	quote::quote,
	syn::parse_quote,
};

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(formattable))]
struct VariantFieldAttrs {
	#[as_ref]
	_ident: Option<syn::Ident>,
	#[as_ref]
	ty:     syn::Type,
}

#[derive(Debug, darling::FromVariant, derive_more::AsRef)]
#[darling(attributes(formattable))]
struct VariantAttrs {
	#[as_ref]
	ident:  syn::Ident,
	#[as_ref]
	fields: darling::ast::Fields<VariantFieldAttrs>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(formattable))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,
}

#[derive(Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(formattable))]
struct Attrs {
	#[as_ref]
	ident:    syn::Ident,
	#[as_ref]
	generics: syn::Generics,
	#[as_ref]
	data:     darling::ast::Data<VariantAttrs, FieldAttrs>,
}

pub fn derive(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> {
	let input = syn::parse::<syn::DeriveInput>(input).context("Unable to parse input")?;

	let attrs = Attrs::from_derive_input(&input).context("Unable to parse attributes")?;
	let item_ident = &attrs.ident;

	// Parse body, parsable impl and error enum (with it's impls)
	let impls = match &attrs.data {
		darling::ast::Data::Enum(variants) => self::derive_enum(variants),
		darling::ast::Data::Struct(fields) => self::derive_struct(fields),
	};

	let Impls {
		with_strings,
		with_prefix_ws,
	} = impls;

	let impl_generics = util::with_bounds(&attrs, |ty| parse_quote! { #ty: rustidy_format::Formattable });
	let (impl_generics, ty_generics, impl_where_clause) = impl_generics.split_for_impl();
	let output = quote! {
		#[automatically_derived]
		impl #impl_generics rustidy_format::Formattable for #item_ident #ty_generics #impl_where_clause {
			fn with_prefix_ws<WITH_PREFIX_WS_O>(
				&mut self,
				ctx: &mut rustidy_format::Context,
				f: &mut impl FnMut(&mut rustidy_util::Whitespace, &mut rustidy_format::Context) -> WITH_PREFIX_WS_O,
			) -> Option<WITH_PREFIX_WS_O> {
				#with_prefix_ws
			}

			fn with_strings<WITH_STRINGS_O>(
				&mut self,
				ctx: &mut rustidy_format::Context,
				mut exclude_prefix_ws: bool,
				f: &mut impl FnMut(&mut rustidy_util::AstStr, &mut rustidy_format::Context) -> std::ops::ControlFlow<WITH_STRINGS_O>,
			) -> std::ops::ControlFlow<WITH_STRINGS_O> {
				#with_strings
			}
		}
	};

	Ok(output.into())
}

fn derive_enum(variants: &[VariantAttrs]) -> Impls<syn::Expr, syn::Expr> {
	let variant_impls = variants
		.iter()
		.map(|variant| {
			let variant_ident = &variant.ident;
			let with_strings =
				parse_quote! { Self::#variant_ident(ref mut value) => rustidy_format::Formattable::with_strings(value, ctx, exclude_prefix_ws, f), };

			let with_prefix_ws =
				parse_quote! { Self::#variant_ident(ref mut value) => value.with_prefix_ws(ctx, f), };

			Impls {
				with_strings,
				with_prefix_ws,
			}
		})
		.collect::<Impls<Vec<syn::Arm>, Vec<syn::Arm>>>();


	let Impls {
		with_strings,
		with_prefix_ws,
	} = variant_impls;
	let with_strings = parse_quote! { match *self { #( #with_strings )* } };
	let with_prefix_ws = parse_quote! { match *self { #( #with_prefix_ws )* } };

	Impls {
		with_strings,
		with_prefix_ws,
	}
}

fn derive_struct(fields: &darling::ast::Fields<FieldAttrs>) -> Impls<syn::Expr, syn::Expr> {
	let Impls {
		with_strings,
		with_prefix_ws,
	} = fields
		.iter()
		.enumerate()
		.map(|(field_idx, field)| self::derive_struct_field(field_idx, field))
		.collect::<Impls<Vec<_>, Vec<_>>>();

	let with_strings = parse_quote! {{
		#( #with_strings; )*
		std::ops::ControlFlow::Continue(())
	}};

	let with_prefix_ws = parse_quote! {{
		#( #with_prefix_ws; )*
		None
	}};

	Impls {
		with_strings,
		with_prefix_ws,
	}
}

fn derive_struct_field(field_idx: usize, field: &FieldAttrs) -> Impls<syn::Expr, syn::Expr> {
	let field_ident = util::field_member_access(field_idx, field);

	let with_strings = parse_quote! {{
		rustidy_format::Formattable::with_strings(&mut self.#field_ident, ctx, exclude_prefix_ws, f)?;

		// If this field wasn't empty, then we no longer exclude the prefix ws, since
		// we already excluded it here.
		if exclude_prefix_ws && !rustidy_format::Formattable::is_empty(&mut self.#field_ident, ctx, false) {
			exclude_prefix_ws = false;
		}
	}};

	let with_prefix_ws = parse_quote! {{
		// If we used the whitespace, return
		if let Some(value) = rustidy_format::Formattable::with_prefix_ws(&mut self.#field_ident, ctx, f) {
			return Some(value);
		}

		// Otherwise, if this field wasn't empty, we have no more fields
		// to check and we can return
		if !rustidy_format::Formattable::is_empty(&mut self.#field_ident, ctx, false) {
			return None;
		}
	}};

	Impls {
		with_strings,
		with_prefix_ws,
	}
}

#[derive(Default, Debug)]
struct Impls<WithStrings, PrefixWs> {
	with_strings:   WithStrings,
	with_prefix_ws: PrefixWs,
}

impl<T0, T1, A0, A1> FromIterator<Impls<A0, A1>> for Impls<T0, T1>
where
	T0: Default + Extend<A0>,
	T1: Default + Extend<A1>,
{
	fn from_iter<I: IntoIterator<Item = Impls<A0, A1>>>(iter: I) -> Self {
		let mut output = Self::default();
		for impls in iter {
			output.with_strings.extend_one(impls.with_strings);
			output.with_prefix_ws.extend_one(impls.with_prefix_ws);
		}

		output
	}
}
