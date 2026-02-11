//! `derive(ParseError)`

// Imports
use {
	crate::util::{self, Fmt, IteratorTryUnzip},
	app_error::{AppError, Context, bail, ensure},
	darling::FromDeriveInput,
	itertools::Itertools,
	proc_macro2::Span,
	quote::quote,
	syn::parse_quote,
};

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(parse_error))]
struct VariantFieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,

	#[darling(default)]
	source: bool,
}

#[derive(Debug, darling::FromVariant, derive_more::AsRef)]
#[darling(attributes(parse_error))]
struct VariantAttrs {
	#[as_ref]
	ident:  syn::Ident,
	#[as_ref]
	fields: darling::ast::Fields<VariantFieldAttrs>,

	#[darling(default)]
	fatal: bool,

	#[darling(default)]
	multiple: bool,

	#[darling(default)]
	transparent: bool,

	fmt: Option<Fmt>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(parse_error))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,
}

#[derive(Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(parse_error))]
struct Attrs {
	#[as_ref]
	ident:    syn::Ident,
	#[as_ref]
	generics: syn::Generics,
	#[as_ref]
	data:     darling::ast::Data<VariantAttrs, FieldAttrs>,

	#[darling(default)]
	fatal: bool,

	#[darling(default)]
	transparent: bool,

	fmt: Option<Fmt>,
}

pub fn derive(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> {
	let input = syn::parse::<syn::DeriveInput>(input).context("Unable to parse input")?;

	let attrs = Attrs::from_derive_input(&input).context("Unable to parse attributes")?;
	let item_ident = &attrs.ident;

	let is_item_transparent = attrs.transparent;
	let transparent_field_access = match is_item_transparent {
		true => {
			let darling::ast::Data::Struct(fields) = &attrs.data else {
				bail!("Cannot set `#[parse_error(transparent)]` on enums or unions");
			};
			let field = fields
				.fields
				.iter()
				.exactly_one()
				.context("`#[parse_error(transparent)]` is only supported for single-field structs")?;

			let field_ident = util::field_member_access(0, field);
			Some((field, quote! { self.#field_ident }))
		},
		false => None,
	};

	let item_error_fmt = &attrs.fmt;


	let mut impl_generics = attrs.generics.clone();
	let impl_where_clause = impl_generics.make_where_clause();
	let (is_fatal, pos, to_app_error) = match &attrs.data {
		darling::ast::Data::Enum(variants) => {
			// If we have generics, add bounds
			if !attrs.generics.params.is_empty() {
				for variant in variants {
					for field in &variant.fields.fields {
						let ty = &field.ty;
						impl_where_clause
							.predicates
							.push(parse_quote! { #ty: rustidy_parse::ParseError });
					}
				}
			}

			let (is_fatal_variants, pos_variants) = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;

					ensure!(
						!(variant.transparent && variant.multiple),
						"Error variant cannot be transparent and multiple at the same time"
					);

					let res = match variant.multiple {
						true => {
							let fields_ident = variant
								.fields
								.iter()
								.map(|variant_field| {
									variant_field
										.ident
										.as_ref()
										.context("`#[parse_error(multiple)]` is only supported on named variants")
								})
								.collect::<Result<Vec<_>, _>>()?;

							let variants_pos = fields_ident
								.iter()
								.map(|field_ident| {
									quote! { let #field_ident = #field_ident.pos(); }
								})
								.collect::<Vec<_>>();

							match fields_ident.is_empty() {
								true => (quote! { Self::#variant_ident {} => false, }, quote! { Self::#variant_ident {} => None, }),
								false => {
									let is_fatal = quote! { Self::#variant_ident { #( ref #fields_ident, )* } => #( #fields_ident.is_fatal() )||*, };

									let pos = quote! { Self::#variant_ident { #( ref #fields_ident, )* } => {
										#( #variants_pos )*

										[ #( #fields_ident, )* ]
											.into_iter()
											.flatten()
											.max()
									}, };

									(is_fatal, pos)
								},
							}
						},
						false => {
							let field = match variant.transparent {
								true => {
									let field = variant.fields.iter().enumerate().exactly_one().context("Exactly 1 field must exist on `#[parse_error(transparent)]` variants")?;
									Some(field)
								},
								false => variant
									.fields
									.iter()
									.enumerate()
									.filter(|(_, variant_field)| {
										variant_field.source
									})
									.at_most_one()
									.context("At most 1 field may have `#[parse_error(source)]`")?,
							};

							let is_fatal = variant.fatal;
							match field {
								Some((field_idx, field)) => match &field.ident {
									Some(field_ident) => (
										quote! { Self::#variant_ident { ref #field_ident, .. } => #is_fatal || #field_ident.is_fatal(), },
										quote! { Self::#variant_ident { ref #field_ident, .. } => #field_ident.pos(), },
									),
									None => {
										ensure!(
											field_idx == 0,
											"Non-first unnamed `#[parse_error(source)]` aren't supported yet"
										);

										let is_fatal =
											quote! { Self::#variant_ident(ref err, ..) => #is_fatal || err.is_fatal(), };
										let pos = quote! { Self::#variant_ident(ref err, ..) => err.pos(), };

										(is_fatal, pos)
									},
								},
								None => {
									let is_fatal = quote! { Self::#variant_ident { .. } => #is_fatal, };
									let pos = quote! { Self::#variant_ident { .. } => None, };

									(is_fatal, pos)
								},
							}
						},
					};

					Ok::<_, AppError>(res)
				})
				.try_unzip::<Vec<_>, Vec<_>>()?;

			let is_fatal = quote! { match *self {
				#(#is_fatal_variants)*
			} };

			let pos = quote! { match *self {
				#( #pos_variants )*
			} };

			let to_app_error_variants = itertools::izip!(variants)
				.map(|variant| {
					let variant_ident = &variant.ident;

					let field_idents = variant
						.fields
						.fields
						.iter()
						.enumerate()
						.map(|(variant_field_idx, variant_field)| match &variant_field.ident {
							Some(ident) => ident.clone(),
							None => syn::Ident::new(&format!("_{variant_field_idx}"), Span::mixed_site()),
						})
						.collect::<Vec<_>>();

					let output = match &*field_idents {
						[] => {
							ensure!(!variant.transparent, "Empty variants may not be transparent");
							let Fmt { parts } = variant.fmt.as_ref().context(
								"Expected either `#[parse_error(transparent)]` or `#[parse_error(fmt = \"...\")]`",
							)?;

							quote! {
								match format_args!(#( #parts, )*).as_str() {
									Some(fmt) => app_error::AppError::msg(fmt),
									None => app_error::AppError::fmt(format!(#( #parts, )*)),
								}
							}
						},
						[field_ident] => {
							quote! { rustidy_parse::ParseError::to_app_error(#field_ident, parser) }
						},
						_ => quote! { app_error::AppError::from_multiple([
							#( rustidy_parse::ParseError::to_app_error(#field_idents, parser), )*
						]) },
					};

					let pat = match variant.fields.style {
						_ if field_idents.is_empty() => quote! { {} },
						darling::ast::Style::Unit => unreachable!("Unit should be empty"),

						darling::ast::Style::Tuple => quote! { (#( ref #field_idents, )*) },

						darling::ast::Style::Struct => quote! { { #( ref #field_idents, )* } },
					};

					Ok(quote! { Self::#variant_ident #pat => #output, })
				})
				.collect::<Result<Vec<_>, AppError>>()?;

			let to_app_error = quote! {
				match *self {
					#( #to_app_error_variants )*
				}
			};

			(is_fatal, pos, to_app_error)
		},

		darling::ast::Data::Struct(fields) => {
			let is_fatal = attrs.fatal;
			let is_fatal = match &transparent_field_access {
				Some((_, field)) => quote! { #is_fatal || #field.is_fatal() },
				None => quote! { #is_fatal },
			};

			let pos = match &transparent_field_access {
				Some((_, field)) => quote! { #field.pos() },
				None => quote! { None },
			};

			let to_app_error = match &transparent_field_access {
				Some((field, field_access)) => {
					// With transparent fields, we need a type bound if we're generic
					if !attrs.generics.params.is_empty() {
						let ty = &field.ty;
						impl_where_clause
							.predicates
							.push(parse_quote! { #ty: rustidy_parse::ParseError });
					}

					quote! { #field_access.to_app_error(parser) }
				},
				None => {
					let Fmt { parts } = item_error_fmt
						.as_ref()
						.context("Expected either `#[parse_error(transparent)]` or `#[parse_error(fmt = \"...\")]`")?;

					let field_idents = fields
						.fields
						.iter()
						.enumerate()
						.map(|(field_idx, field)| util::field_member_access(field_idx, field))
						.collect::<Vec<_>>();


					quote! {
						let Self { #( #field_idents, )* } = self;

						match format_args!(#( #parts, )*).as_str() {
							Some(fmt) => app_error::AppError::msg(fmt),
							None => app_error::AppError::fmt(format!(#( #parts, )*)),
						}
					}
				},
			};

			(is_fatal, pos, to_app_error)
		},
	};

	let (impl_generics, ty_generics, where_clause) = impl_generics.split_for_impl();
	let output = quote! {
		#[automatically_derived]
		impl #impl_generics rustidy_parse::ParseError for #item_ident #ty_generics #where_clause {
			fn is_fatal(&self) -> bool {
				#is_fatal
			}

			fn pos(&self) -> Option<rustidy_util::AstPos> {
				#pos
			}

			fn to_app_error(&self, parser: &rustidy_parse::Parser) -> app_error::AppError {
				#to_app_error
			}
		}
	};

	Ok(output.into())
}
