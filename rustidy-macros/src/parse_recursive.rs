//! `derive(ParseRecursive)`

// Imports
use {
	app_error::{AppError, Context, bail, ensure},
	darling::FromDeriveInput,
	itertools::Itertools,
	quote::quote,
	syn::parse_quote,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, darling::FromMeta)]
enum Kind {
	Left,
	Right,
	Fully,
}

#[derive(Clone, Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(parse_recursive))]
struct VariantFieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,
}

impl quote::ToTokens for VariantFieldAttrs {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let Self { ident, ty } = self;
		match ident {
			Some(ident) => quote! { #ident: #ty },
			None => quote! { #ty },
		}.to_tokens(tokens);
	}
}

#[derive(Clone, Debug, darling::FromVariant, derive_more::AsRef)]
#[darling(attributes(parse_recursive))]
struct VariantAttrs {
	ident:     syn::Ident,
	fields:    darling::ast::Fields<VariantFieldAttrs>,

	#[darling(default)]
	recursive: bool,
}

impl quote::ToTokens for VariantAttrs {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let Self { ident, fields, recursive: _, } = self;
		quote! { #ident #fields }.to_tokens(tokens);
	}
}

#[derive(Clone, Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(parse_recursive))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,
}

impl quote::ToTokens for FieldAttrs {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let Self { ident, ty } = self;
		match ident {
			Some(ident) => quote! { #ident: #ty },
			None => quote! { #ty },
		}.to_tokens(tokens);
	}
}

#[derive(Clone, Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(parse_recursive))]
struct Attrs {
	ident:       syn::Ident,
	generics:    syn::Generics,
	data:        darling::ast::Data<VariantAttrs, FieldAttrs>,

	#[darling(default)]
	transparent: bool,

	root:        syn::TypePath,
	into_root:   Option<syn::TypePath>,
	// TODO: We should allow multiple here
	skip_if_tag: Option<syn::Expr>,

	kind:        Option<Kind>,
}

pub fn derive(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> {
	let input = syn::parse::<syn::DeriveInput>(input)
		.context("Unable to parse input")?;

	let attrs = Attrs::from_derive_input(&input)
		.context("Unable to parse attributes")?;
	let item_ident = &attrs.ident;

	let ident_base = syn::Ident::new(&format!("{item_ident}Base"), item_ident.span());
	let ident_prefix = syn::Ident::new(
		&format!("{item_ident}Prefix"),
		item_ident.span()
	);
	let ident_infix = syn::Ident::new(&format!("{item_ident}Infix"), item_ident.span());
	let ident_suffix = syn::Ident::new(
		&format!("{item_ident}Suffix"),
		item_ident.span()
	);

	let root_ty = &attrs.root;

	// TODO: Generics
	let root_conversion_impls = attrs.into_root.as_ref().map(|into_root_ty| {
		quote! {
			#[automatically_derived]
			impl rustidy_parse::IntoRecursiveRoot<#root_ty> for #item_ident {
				fn into_recursive_root(self, parser: &mut rustidy_parse::Parser) -> #root_ty {
					<#into_root_ty as rustidy_parse::IntoRecursiveRoot<#root_ty>>::into_recursive_root(
						<#into_root_ty>::from(self),
						parser
					)
				}
			}

			#[automatically_derived]
			impl rustidy_parse::TryFromRecursiveRoot<#root_ty> for #item_ident {
				fn try_from_recursive_root(root: #root_ty, parser: &mut rustidy_parse::Parser) -> Option<Self> {
					let into_root = <#into_root_ty as rustidy_parse::TryFromRecursiveRoot<#root_ty>>::try_from_recursive_root(
						root,
						parser
					)?;
					Self::try_from(into_root).ok()
				}
			}
		}
	});

	if attrs.transparent {
		return self::emit_transparent(&attrs, root_conversion_impls.as_ref());
	}

	let skip_if_tag = attrs.skip_if_tag.as_ref();
	let skip_if_tag_attr = skip_if_tag
		.map(|tag| quote! { #[parse(skip_if_tag = #tag)] });

	let impls = match &attrs.data {
		darling::ast::Data::Enum(variants) => {
			let recursive_variants = variants
				.iter()
				.filter(|variant| variant.recursive)
				.cloned().collect::<Vec<_>>();

			let mut suffix_variants = recursive_variants.clone();
			let suffix_match_arms = suffix_variants
				.iter()
				.map(|variant| try {
					let field = self::get_variant_as_unnamed_single(variant)
						.context(
							"Enum variants must be tuple variants with a single field"
						)?;

					let ty = &field.ty;
					let variant_ident = &variant.ident;

					quote! { #ident_suffix::#variant_ident(suffix) => Self::#variant_ident(<#ty as rustidy_parse::ParsableRecursive<#root_ty>>::join_suffix(root, suffix, parser)), }
				})
				.collect::<Result<Vec<_>, _>>()?;
			for variant in &mut suffix_variants {
				let field = self::get_variant_as_unnamed_single_mut(variant)
					.context(
						"Enum variants must be tuple variants with a single field"
					)?;
				let ty = &field.ty;
				field.ty = parse_quote! { <#ty as rustidy_parse::ParsableRecursive<#root_ty>>::Suffix };
			}

			let suffix_impl = quote! {
				type Suffix = #ident_suffix;

				#[allow(unreachable_code)]
				fn join_suffix(root: #root_ty, suffix: Self::Suffix, parser: &mut rustidy_parse::Parser) -> Self {
					match suffix {
						#( #suffix_match_arms )*
					}
				}
			};

			let suffix_ty = quote! {
				#[derive(Debug, rustidy_parse::Parse)]
				#skip_if_tag_attr
				pub enum #ident_suffix {
					#( #suffix_variants, )*
				}
			};

			let mut prefix_variants = recursive_variants.clone();
			let prefix_match_arms = prefix_variants
				.iter()
				.map(|variant| try {
					let field = self::get_variant_as_unnamed_single(variant)
						.context(
							"Enum variants must be tuple variants with a single field"
						)?;

					let ty = &field.ty;
					let variant_ident = &variant.ident;

					quote! { #ident_prefix::#variant_ident(prefix) => Self::#variant_ident(<#ty as rustidy_parse::ParsableRecursive<#root_ty>>::join_prefix(prefix, root, parser)), }
				})
				.collect::<Result<Vec<_>, AppError>>()?;
			for variant in &mut prefix_variants {
				let field = self::get_variant_as_unnamed_single_mut(variant)
					.context(
						"Enum variants must be tuple variants with a single field"
					)?;
				let ty = &field.ty;
				field.ty = parse_quote! { <#ty as rustidy_parse::ParsableRecursive<#root_ty>>::Prefix };
			}

			let prefix_impl = quote! {
				type Prefix = #ident_prefix;

				#[allow(unreachable_code)]
				fn join_prefix(prefix: Self::Prefix, root: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
					match prefix {
						#( #prefix_match_arms )*
					}
				}
			};
			let prefix_ty = quote! {
				#[derive(Debug, rustidy_parse::Parse)]
				#skip_if_tag_attr
				pub enum #ident_prefix {
					#( #prefix_variants, )*
				}
			};

			let mut infix_variants = recursive_variants.clone();
			let infix_match_arms = infix_variants
				.iter()
				.map(|variant| try {
					let field = self::get_variant_as_unnamed_single(variant)
						.context(
							"Enum variants must be tuple variants with a single field"
						)?;

					let ty = &field.ty;
					let variant_ident = &variant.ident;

					quote! { #ident_infix::#variant_ident(infix) => Self::#variant_ident(<#ty as rustidy_parse::ParsableRecursive<#root_ty>>::join_infix(lhs, infix, rhs, parser)), }
				})
				.collect::<Result<Vec<_>, AppError>>()?;
			for variant in &mut infix_variants {
				let field = self::get_variant_as_unnamed_single_mut(variant)
					.context(
						"Enum variants must be tuple variants with a single field"
					)?;
				let ty = &field.ty;
				field.ty = parse_quote! { <#ty as rustidy_parse::ParsableRecursive<#root_ty>>::Infix };
			}

			let infix_impl = quote! {
				type Infix = #ident_infix;

				#[allow(unreachable_code)]
				fn join_infix(lhs: #root_ty, infix: Self::Infix, rhs: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
					match infix {
						#( #infix_match_arms )*
					}
				}
			};

			let infix_ty = quote! {
				#[derive(Debug, rustidy_parse::Parse)]
				#skip_if_tag_attr
				pub enum #ident_infix {
					#( #infix_variants, )*
				}
			};

			let mut base_variants = recursive_variants.clone();
			let mut base_match_arms = base_variants
				.iter()
				.map(|variant| try {
					let field = self::get_variant_as_unnamed_single(variant)
						.context(
							"Enum variants must be tuple variants with a single field"
						)?;

					let ty = &field.ty;
					let variant_ident = &variant.ident;

					quote! { #ident_base::#variant_ident(base) => Self::#variant_ident(<#ty as rustidy_parse::ParsableRecursive<#root_ty>>::from_base(base, parser)), }
				})
				.collect::<Result<Vec<_>, AppError>>()?;
			for variant in &mut base_variants {
				let field = self::get_variant_as_unnamed_single_mut(variant)
					.context(
						"Enum variants must be tuple variants with a single field"
					)?;
				let ty = &field.ty;
				field.ty = parse_quote! { <#ty as rustidy_parse::ParsableRecursive<#root_ty>>::Base };
			}

			for variant in variants {
				if recursive_variants.iter().any(
					|existing_variant| existing_variant.ident == variant.ident
				) {
					continue;
				}

				base_variants.push(variant.clone());

				let variant_ident = &variant.ident;
				base_match_arms.push(quote! {
					#ident_base::#variant_ident(base) => Self::#variant_ident(base),
				});
			}

			let impl_base = quote! {
				type Base = #ident_base;

				#[allow(unreachable_code)]
				fn from_base(base: Self::Base, parser: &mut rustidy_parse::Parser) -> Self {
					match base {
						#( #base_match_arms )*
					}
				}
			};
			let ty_base = quote! {
				#[derive(Debug, rustidy_parse::Parse)]
				#skip_if_tag_attr
				pub enum #ident_base {
					#( #base_variants, )*
				}
			};

			quote! {
				#[automatically_derived]
				impl rustidy_parse::ParsableRecursive<#root_ty> for #item_ident {
					#suffix_impl
					#prefix_impl
					#infix_impl
					#impl_base
				}

				#root_conversion_impls

				#suffix_ty
				#prefix_ty
				#infix_ty
				#ty_base
			}
		},
		darling::ast::Data::Struct(fields) => {
			let kind = attrs.kind.context(
				"Expected `#[parse_recursive(kind = <kind>)]` with `kind` equal to `left`, `right` or `fully`",
			)?;

			let (suffix_impl, suffix_ty) = match kind {
				Kind::Left => {
					let field = fields
						.iter()
						.next()
						.context("Expected at least 1 field")?;
					let suffix_fields = fields.iter().skip(1).collect::<Vec<_>>();

					let join = match &fields.style {
						darling::ast::Style::Struct => {
							let field_ident = field
								.ident
								.as_ref()
								.context("Should have an ident")?;
							let field_ty = &field.ty;
							let suffix_fields_ident = suffix_fields
								.iter()
								.map(
									|field| field
										.ident
										.as_ref()
										.context("Should have an ident")
								)
								.collect::<Result<Vec<_>, _>>()?;

							quote! {
								Self {
									#field_ident: <#field_ty as rustidy_parse::FromRecursiveRoot<#root_ty>>::from_recursive_root(lhs, parser),
									#( #suffix_fields_ident: suffix.#suffix_fields_ident, )*
								}
							}
						},
						darling::ast::Style::Tuple => bail!("Tuple structs aren't supported yet"),
						darling::ast::Style::Unit => unreachable!(),
					};

					let impl_ = quote! {
						type Suffix = #ident_suffix;

						fn join_suffix(lhs: #root_ty, suffix: Self::Suffix, parser: &mut rustidy_parse::Parser) -> Self {
							#join
						}
					};

					let ty = quote! {
						#[derive(Debug, rustidy_parse::Parse)]
						#skip_if_tag_attr
						pub struct #ident_suffix {
							#( #suffix_fields, )*
						}
					};

					(impl_, Some(ty))
				},
				_ => (quote! {
						type Suffix = !;

						fn join_suffix(lhs: #root_ty, suffix: Self::Suffix, parser: &mut rustidy_parse::Parser) -> Self {
							suffix
						}
					}, None,),
			};

			let (prefix_impl, prefix_ty) = match kind {
				Kind::Right => {
					let field = fields
						.iter()
						.last()
						.context("Expected at least 1 field")?;
					let prefix_fields = fields
						.iter()
						.take(fields.len() - 1)
						.collect::<Vec<_>>();

					let join = match &fields.style {
						darling::ast::Style::Struct => {
							let field_ident = field
								.ident
								.as_ref()
								.context("Should have an ident")?;
							let field_ty = &field.ty;
							let prefix_fields_ident = prefix_fields
								.iter()
								.map(
									|field| field
										.ident
										.as_ref()
										.context("Should have an ident")
								)
								.collect::<Result<Vec<_>, _>>()?;

							quote! {
								Self {
									#field_ident: <#field_ty as rustidy_parse::FromRecursiveRoot<#root_ty>>::from_recursive_root(lhs, parser),
									#( #prefix_fields_ident: prefix.#prefix_fields_ident, )*
								}
							}
						},
						darling::ast::Style::Tuple => bail!("Tuple structs aren't supported yet"),
						darling::ast::Style::Unit => unreachable!(),
					};

					let impl_ = quote! {
						type Prefix = #ident_prefix;

						fn join_prefix(prefix: Self::Prefix, lhs: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
							#join
						}
					};

					let ty = quote! {
						#[derive(Debug, rustidy_parse::Parse)]
						#skip_if_tag_attr
						pub struct #ident_prefix {
							#( #prefix_fields, )*
						}
					};

					(impl_, Some(ty))
				},
				_ => (quote! {
						type Prefix = !;

						fn join_prefix(prefix: Self::Prefix, _: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
							prefix
						}
					}, None,),
			};

			let (infix_impl, infix_ty) = match kind {
				Kind::Fully => {
					ensure!(fields.len() >= 2, "Expected at least 2 fields");
					let lhs_field = fields
						.iter()
						.next()
						.context("Expected at least 2 field")?;
					let rhs_field = fields
						.iter()
						.last()
						.context("Expected at least 2 field")?;

					let infix_fields = fields
						.iter()
						.skip(1)
						.take(fields.len() - 2)
						.collect::<Vec<_>>();

					let join = match &fields.style {
						darling::ast::Style::Struct => {
							let lhs_field_ident = lhs_field
								.ident
								.as_ref()
								.context("Should have an ident")?;
							let rhs_field_ident = rhs_field
								.ident
								.as_ref()
								.context("Should have an ident")?;
							let lhs_field_ty = &lhs_field.ty;
							let rhs_field_ty = &rhs_field.ty;
							let infix_fields_ident = infix_fields
								.iter()
								.map(
									|field| field
										.ident
										.as_ref()
										.context("Should have an ident")
								)
								.collect::<Result<Vec<_>, _>>()?;

							quote! {
								Self {
									#lhs_field_ident: <#lhs_field_ty as rustidy_parse::FromRecursiveRoot<#root_ty>>::from_recursive_root(lhs, parser),
									#rhs_field_ident: <#rhs_field_ty as rustidy_parse::FromRecursiveRoot<#root_ty>>::from_recursive_root(rhs, parser),
									#( #infix_fields_ident: infix.#infix_fields_ident, )*
								}
							}
						},
						darling::ast::Style::Tuple => bail!("Tuple structs aren't supported yet"),
						darling::ast::Style::Unit => unreachable!(),
					};

					let impl_ = quote! {
						type Infix = #ident_infix;

						fn join_infix(lhs: #root_ty, infix: Self::Infix, rhs: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
							#join
						}
					};

					let ty = quote! {
						#[derive(Debug, rustidy_parse::Parse)]
						#skip_if_tag_attr
						pub struct #ident_infix {
							#( #infix_fields, )*
						}
					};

					(impl_, Some(ty))
				},
				_ => (quote! {
						type Infix = !;

						fn join_infix(_: #root_ty, infix: Self::Infix, _: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
							infix
						}
					}, None,),
			};

			quote! {
				#[automatically_derived]
				impl rustidy_parse::ParsableRecursive<#root_ty> for #item_ident {
					type Base = !;

					fn from_base(base: Self::Base, parser: &mut rustidy_parse::Parser) -> Self {
						base
					}

					#prefix_impl
					#suffix_impl
					#infix_impl
				}

				#root_conversion_impls

				#prefix_ty
				#suffix_ty
				#infix_ty
			}
		},
	};

	let output = quote! {
		#impls
	};

	Ok(output.into())
}

/// Emits a transparent derive
fn emit_transparent(
	attrs: &Attrs,
	root_conversion_impls: Option<&proc_macro2::TokenStream>,
) -> Result<proc_macro::TokenStream, AppError> {
	let darling::ast::Data::Struct(fields) = &attrs.data else {
		app_error::bail!("`#[parse_recursive(transparent)]` is only supported on structs")
	};

	let item_ident = &attrs.ident;
	let root_ty = &attrs.root;

	let field = fields.fields.iter().exactly_one().context(
		"`#[parse_recursive(transparent)]` expects a single field"
	)?;
	let field_ty = &field.ty;

	let output = quote! {
		#[automatically_derived]
		impl rustidy_parse::ParsableRecursive<#root_ty> for #item_ident {
			type Prefix = <#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::Prefix;
			type Base   = <#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::Base;
			type Suffix = <#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::Suffix;
			type Infix  = <#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::Infix;

			fn join_prefix(prefix: Self::Prefix, root: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
				Self(<#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::join_prefix(prefix, root, parser))
			}
			fn from_base(base: Self::Base, parser: &mut rustidy_parse::Parser) -> Self {
				Self(<#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::from_base(base, parser))
			}
			fn join_suffix(root: #root_ty, suffix: Self::Suffix, parser: &mut rustidy_parse::Parser) -> Self {
				Self(<#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::join_suffix(root, suffix, parser))
			}
			fn join_infix(lhs: #root_ty, infix: Self::Infix, rhs: #root_ty, parser: &mut rustidy_parse::Parser) -> Self {
				Self(<#field_ty as rustidy_parse::ParsableRecursive<#root_ty>>::join_infix(lhs, infix, rhs, parser))
			}
		}

		#root_conversion_impls
	};

	Ok(output.into())
}

/// Gets a variant's single unnamed field
fn get_variant_as_unnamed_single(variant: &VariantAttrs) -> Option<&VariantFieldAttrs> {
	if !variant.fields.style.is_tuple() {
		return None;
	}
	variant.fields.fields.first()
}

/// Gets a variant's single unnamed field
fn get_variant_as_unnamed_single_mut(variant: &mut VariantAttrs) -> Option<&mut VariantFieldAttrs> {
	if !variant.fields.style.is_tuple() {
		return None;
	}
	variant.fields.fields.first_mut()
}
