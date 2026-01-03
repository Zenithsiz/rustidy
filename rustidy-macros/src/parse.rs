//! `derive(Parse)`

// Imports
use {
	crate::util,
	app_error::{AppError, Context},
	convert_case::Casing,
	darling::FromDeriveInput,
	itertools::Itertools,
	proc_macro2::Span,
	quote::quote,
	std::collections::HashMap,
	syn::{parse_quote, punctuated::Punctuated},
};

#[derive(Debug, darling::FromMeta)]
struct ExtraErrorVariant {
	name:  syn::Ident,
	#[darling(with = "darling::util::parse_expr::preserve_str_literal")]
	fmt:   syn::Expr,
	#[darling(default)]
	fatal: bool,
}

impl quote::ToTokens for ExtraErrorVariant {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let Self { name, fmt, fatal } = self;
		let fatal = fatal.then(|| quote! { #[parse_error(fatal)] });
		quote! {
			#[parse_error(fmt = #fmt)]
			#fatal
			#name,
		}
		.to_tokens(tokens);
	}
}

#[derive(Debug)]
struct PeekAttrs(syn::Type);

impl darling::FromMeta for PeekAttrs {
	fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
		// TODO: Something better than this...
		Ok(Self(parse_quote! { #expr }))
	}
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(parse))]
struct VariantFieldAttrs {
	#[as_ref]
	_ident: Option<syn::Ident>,
	#[as_ref]
	ty:     syn::Type,
}

#[derive(Debug, darling::FromVariant, derive_more::AsRef)]
#[darling(attributes(parse))]
struct VariantAttrs {
	#[as_ref]
	ident:  syn::Ident,
	#[as_ref]
	fields: darling::ast::Fields<VariantFieldAttrs>,

	#[darling(multiple)]
	peek: Vec<PeekAttrs>,

	#[darling(default)]
	without_tags: bool,

	#[darling(default)]
	box_error: bool,

	#[darling(multiple)]
	#[darling(with = "darling::util::parse_expr::preserve_str_literal")]
	with_tag: Vec<syn::Expr>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(parse))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,

	#[darling(default)]
	without_tags: bool,

	#[darling(multiple)]
	#[darling(with = "darling::util::parse_expr::preserve_str_literal")]
	with_tag: Vec<syn::Expr>,

	update_with:     Option<syn::Expr>,
	try_update_with: Option<syn::Expr>,

	#[darling(default)]
	box_error: bool,

	#[darling(default)]
	fatal: bool,

	// TODO: We should allow multiple here
	skip_if_tag: Option<syn::LitStr>,
}

#[derive(Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(parse))]
struct Attrs {
	#[as_ref]
	ident:    syn::Ident,
	#[as_ref]
	generics: syn::Generics,
	#[as_ref]
	data:     darling::ast::Data<VariantAttrs, FieldAttrs>,

	#[darling(multiple)]
	try_with: Vec<syn::Expr>,

	name:        Option<syn::LitStr>,
	from:        Option<syn::Path>,
	#[darling(multiple)]
	error:       Vec<ExtraErrorVariant>,
	// TODO: We should allow multiple here
	skip_if_tag: Option<syn::LitStr>,
}

pub fn derive(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> {
	let input = syn::parse::<syn::DeriveInput>(input).context("Unable to parse input")?;

	let attrs = Attrs::from_derive_input(&input).context("Unable to parse attributes")?;
	let item_ident = &attrs.ident;


	// Error type identifier
	let error_ident = syn::Ident::new(&format!("{item_ident}Error"), item_ident.span());

	let name_coverage = attrs.name.as_ref().map(|_| quote! { #[coverage(off)] });
	let name = match &attrs.name {
		Some(name) => quote! { Some(#name) },
		None => quote! { None::<!> },
	};

	let skip_if_tag_err_variant_ident = syn::Ident::new("Tag", Span::mixed_site());
	let skip_if_tag_expr = attrs.skip_if_tag.as_ref().map(|tag| {
		quote! {
			if parser.has_tag(#tag) {
				return Err(#error_ident::#skip_if_tag_err_variant_ident);
			}
		}
	});

	let skip_if_tag_err_variant = attrs.skip_if_tag.as_ref().map(|tag| {
		let fmt_msg = format!("Tag `{}` was present", tag.value());
		quote! {
			#[parse_error(fmt = #fmt_msg)]
			#skip_if_tag_err_variant_ident,
		}
	});

	// Parse body, parsable impl and error enum (with it's impls)
	// TODO: Instead of getting the whole error enum here, we should just
	//       get the variants so we can reduce duplication in adding skip tag/
	//       extra error variants.
	let (parse_body, error_enum) = match &attrs.from {
		Some(from) => {
			// TODO: Support tags
			let body = quote! {
				#skip_if_tag_expr

				let value = parser
					.parse::<#from>()
					.map_err(#error_ident::From)?;

				Ok(crate::parser::ParsableFrom::from_parsable(value))
			};

			let generics = &attrs.generics;
			let extra_variants = &attrs.error;
			let error_enum = quote! {
				#[derive(derive_more::Debug, crate::parser::ParseError)]
				pub enum #error_ident #generics {
					#[parse_error(transparent)]
					From(crate::parser::ParserError<#from>),

					#skip_if_tag_err_variant

					#( #extra_variants )*
				}
			};

			(body, error_enum)
		},

		None => match &attrs.data {
			darling::ast::Data::Enum(variants) => {
				// TODO: Support tags
				let unknown_error_name = syn::Ident::new("Unknown", item_ident.span());

				struct Peek<'a> {
					variant:     &'a VariantAttrs,
					ty:          &'a PeekAttrs,
					err_variant: syn::Ident,
				}
				let peeks = variants
					.iter()
					.flat_map(|variant| {
						variant.peek.iter().enumerate().map(|(idx, ty)| {
							let err_variant =
								syn::Ident::new(&format!("{}Peek{idx}", variant.ident), variant.ident.span());
							Peek {
								variant,
								ty,
								err_variant,
							}
						})
					})
					.collect::<Vec<_>>();
				let parse_peeks = peeks.iter().map(|peek| {
					let Peek {
						variant,
						ty: PeekAttrs(ty),
						err_variant,
					} = peek;
					let variant_ident = &variant.ident;

					quote! {
						if parser.peek::<#ty>().map_err(#error_ident::#err_variant)?.is_ok()
						{
							let value = parser.parse().map_err(#error_ident::#variant_ident)?;
							return Ok(Self::#variant_ident(value));
						}
					}
				});

				let variant_tys = variants
					.iter()
					.map(|variant| {
						let field = variant
							.fields
							.iter()
							.exactly_one()
							.unwrap_or_else(|_| panic!("Enum variant must have a single field"));

						&field.ty
					})
					.collect::<Vec<_>>();

				let err_idents = variants
					.iter()
					.map(|variant| {
						let name = variant.ident.to_string().to_case(convert_case::Case::Snake);
						syn::Ident::new(&format!("{name}_err"), item_ident.span())
					})
					.collect::<Vec<_>>();

				let parse_variants = variants
					.iter()
					.zip(&err_idents)
					.map(|(variant, err_ident)| {
						let mut expr = quote! { parser.try_parse() };
						if variant.without_tags {
							expr = quote! { parser.without_tags(|parser| #expr) };
						}

						for tag in &variant.with_tag {
							expr = quote! { parser.with_tag(#tag, |parser| #expr) };
						}

						let box_error = match variant.box_error {
							true => quote! { .map_err(Box::new) },
							false => quote! {},
						};

						let variant_ident = &variant.ident;
						quote! {
							let #err_ident = match #expr #box_error .map_err(#error_ident::#variant_ident)? {
								// Note: This can be unreachable if `value: !`
								#[allow(unreachable_code)]
								Ok(value) => return Ok(Self::#variant_ident(value)),
								Err(err) => err,
							};
						}
					})
					.collect::<Vec<_>>();

				let unknown_errs_create = variants
					.iter()
					.zip(&err_idents)
					.map(|(variant, error_ident)| match variant.box_error {
						true => quote! { #error_ident: Box::new(#error_ident), },
						false => quote! { #error_ident, },
					})
					.collect::<Vec<_>>();

				let body = quote! {
					#skip_if_tag_expr

					#( #parse_peeks )*

					#( #parse_variants )*

					Err(#error_ident::#unknown_error_name { #( #unknown_errs_create )* })
				};

				let error_enum_variants = variants
					.iter()
					.zip(&variant_tys)
					.map(|(variant, variant_ty)| {
						let ty = quote! { crate::parser::ParserError<#variant_ty> };
						let ty = match variant.box_error {
							true => quote! { Box<#ty> },
							false => ty,
						};

						let variant_ident = &variant.ident;
						quote! {
														#[parse_error(transparent)]
							#variant_ident(#ty),
						}
					})
					.chain(peeks.iter().map(|peek| {
						let Peek {
							variant: _,
							ty: PeekAttrs(ty),
							err_variant,
						} = peek;
						let err_ty = quote! { crate::parser::ParserError<#ty> };

						quote! {
														#[parse_error(transparent)]
							#err_variant(#err_ty),
						}
					}))
					.collect::<Vec<_>>();

				let unknown_errs_decl = itertools::izip!(variants, &err_idents, &variant_tys)
					.map(|(variant, err_ident, variant_ty)| {
						let ty = quote! { crate::parser::ParserError<#variant_ty> };
						let ty = match variant.box_error {
							true => quote! { Box<#ty> },
							false => ty,
						};

						quote! {
							#err_ident: #ty,
						}
					})
					.collect::<Vec<_>>();

				let error_generics = util::with_enum_bounds(
					attrs.generics.clone(),
					variants,
					|ty| parse_quote! { #ty: crate::parser::Parse },
				);

				// TODO: Figure out why using just `#error_generics` doesn't work here
				let (impl_generics, _, where_clause) = error_generics.split_for_impl();
				let extra_variants = &attrs.error;
				let error_enum = quote! {
					#[derive(derive_more::Debug, crate::parser::ParseError)]
					pub enum #error_ident #impl_generics #where_clause {
						#( #error_enum_variants )*

						#[parse_error(fmt = "No valid matches")]
						#[parse_error(multiple)]
						#unknown_error_name { #( #unknown_errs_decl )* },

						#skip_if_tag_err_variant

						#( #extra_variants )*
					}
				};


				(body, error_enum)
			},

			darling::ast::Data::Struct(fields) => {
				// TODO: Support top-level tags
				let field_idents = fields
					.fields
					.iter()
					.enumerate()
					.map(|(field_idx, field)| match &field.ident {
						Some(field_ident) => field_ident.clone(),
						None => syn::Ident::new(&format!("_{field_idx}"), Span::mixed_site()),
					})
					.collect::<Punctuated<_, syn::Token![,]>>();

				let error_names = itertools::izip!(&fields.fields, &field_idents)
					.map(|(field, field_ident)| {
						if field.update_with.is_some() || field.try_update_with.is_some() {
							return None;
						}

						let mut name = field_ident.to_string().to_case(convert_case::Case::Pascal);
						if matches!(name.as_str(), "Self") {
							name.push('_');
						}
						if name.starts_with(|ch: char| ch.is_ascii_digit()) {
							name.insert(0, '_');
						}

						Some(syn::Ident::new(&name, field_ident.span()))
					})
					.collect::<Vec<_>>();

				let skip_if_tag_exists_name = itertools::izip!(&fields.fields, &field_idents)
					.filter_map(|(field, field_ident)| {
						let tag = field.skip_if_tag.as_ref()?;
						let error_ident = syn::Ident::new(&format!("tag_exists_{field_ident}"), field_ident.span());

						Some((tag, error_ident))
					})
					.collect::<HashMap<_, _>>();

				let field_tys = fields.fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

				let get_tag_exists = fields
					.fields
					.iter()
					.filter_map(|field| {
						let tag = field.skip_if_tag.as_ref()?;
						let exists_name = &skip_if_tag_exists_name[tag];
						Some(quote! {
							let #exists_name = parser.has_tag(#tag);
						})
					})
					.collect::<Vec<_>>();

				let parse_fields = itertools::izip!(&fields.fields, &error_names, &field_idents)
					.map(|(field, error_name, field_ident)| {
						let mut expr = match &field.try_update_with {
							Some(expr) => {
								assert!(
									field.update_with.is_none(),
									"Cannot specify both `update_with` and `try_update_with`."
								);
								quote! { parser.try_update_with(#expr) }
							},
							None => match &field.update_with {
								Some(expr) => quote! { parser.update_with(#expr) },
								None => quote! { parser.parse() },
							},
						};


						if let Some(tag) = &field.skip_if_tag {
							let exists_name = &skip_if_tag_exists_name[tag];
							expr = quote! {
								match #exists_name {
									true => Ok(Default::default()),
									false => #expr,
								}
							};
						}

						if field.without_tags {
							expr = quote! { parser.without_tags(|parser| #expr) };
						}

						for tag in &field.with_tag {
							expr = quote! { parser.with_tag(#tag, |parser| #expr) };
						}

						let map_err = error_name.as_ref().map(|error_name| {
							let box_error = match field.box_error {
								true => Some(quote! { .map_err(Box::new) }),
								false => None,
							};

							quote! { #box_error .map_err(#error_ident::#error_name) }
						});

						let propagate_error = match field.update_with.is_some() {
							true => None,
							false => Some(quote! { ? }),
						};

						quote! { let #field_ident = #expr #map_err #propagate_error; }
					})
					.collect::<Vec<_>>();

				let body_res = match fields.style {
					darling::ast::Style::Struct => quote! { Self { #field_idents } },
					darling::ast::Style::Tuple => quote! { Self(#field_idents) },
					darling::ast::Style::Unit => quote! { Self },
				};
				let body = quote! {
					#skip_if_tag_expr
					#( #get_tag_exists )*

					#( #parse_fields )*

					Ok(#body_res)
				};

				let fatal_fields = fields
					.fields
					.iter()
					.scan(false, |is_fatal, field| {
						let is_cur_fatal = field.fatal;
						assert!(
							!(*is_fatal && is_cur_fatal),
							"Cannot specify `#[parser(fatal)]` more than once"
						);
						*is_fatal |= is_cur_fatal;

						Some(*is_fatal)
					})
					.collect::<Vec<_>>();

				let error_enum_variants = itertools::izip!(&fields.fields, &error_names, &field_tys, &fatal_fields)
					.filter_map(|(field, error_name, field_ty, is_fatal)| {
						let Some(error_name) = error_name else { return None };

						let fatal = match is_fatal {
							true => quote! { #[parse_error(fatal)] },
							false => quote! {},
						};

						let ty = quote! { crate::parser::ParserError<#field_ty> };
						let ty = match field.box_error {
							true => quote! { Box<#ty> },
							false => ty,
						};

						Some(quote! {
							#[parse_error(transparent)]
							#fatal
							#error_name(#ty),
						})
					})
					.collect::<Vec<_>>();

				// TODO: Figure out why using just `#error_generics` doesn't work here
				let error_generics = util::with_struct_bounds(
					attrs.generics.clone(),
					&fields.fields,
					|ty| parse_quote! { #ty: crate::parser::Parse },
				);
				let (impl_generics, _, where_clause) = error_generics.split_for_impl();
				let extra_variants = &attrs.error;
				let error_enum = quote! {
					#[derive(derive_more::Debug, crate::parser::ParseError)]
					pub enum #error_ident #impl_generics #where_clause {
						#( #error_enum_variants )*

						#skip_if_tag_err_variant

						#( #extra_variants )*
					}
				};

				(body, error_enum)
			},
		},
	};

	let parse_impl = {
		let generics = util::with_bounds(&attrs, |ty| parse_quote! { #ty: crate::parser::Parse });
		let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
		let try_with = attrs.try_with;
		quote! {
			#[automatically_derived]
			impl #impl_generics crate::parser::Parse for #item_ident #ty_generics #where_clause {
				type Error = #error_ident #ty_generics;

				#name_coverage
				fn name() -> Option<impl std::fmt::Display> {
					#name
				}

				#[coverage(on)]
				fn parse_from(parser: &mut crate::parser::Parser) -> Result<Self, Self::Error> {
					#(
						if let Some(value) = (#try_with)(parser)? {
							return Ok(value)
						}
					)*

					#parse_body
				}
			}
		}
	};

	let output = quote! {
		#parse_impl
		#error_enum
	};

	Ok(output.into())
}
