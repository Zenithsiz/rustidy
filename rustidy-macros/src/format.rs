//! `derive(Format)`

// Imports
use {
	crate::util,
	app_error::{AppError, Context, ensure},
	darling::FromDeriveInput,
	quote::quote,
	syn::parse_quote,
};

#[derive(Debug, darling::FromMeta)]
// TODO: Something better than this...
#[darling(from_expr = |expr| Ok(Self { ty: parse_quote! { #expr }, generics: vec![] }))]
struct ArgsTy {
	ty:       syn::Type,

	// TODO: Move these generics to their own attribute
	#[darling(multiple)]
	#[darling(rename = "generic")]
	generics: Vec<syn::TypeParam>,
}

#[derive(Debug, darling::FromMeta)]
#[darling(from_word = || Ok(Self { if_has_tag: None }))]
struct Indent {
	if_has_tag: Option<syn::Expr>,
}

#[derive(Debug, darling::FromMeta)]
#[darling(from_expr = |expr| Ok(Self { tag: expr.clone(), if_: None }))]
struct WithTag {
	tag: syn::Expr,
	if_: Option<syn::Expr>,
}

#[derive(Clone, Debug, darling::FromMeta)]
#[darling(from_expr = |expr| Ok(Self { expr: expr.clone(), if_: None }))]
struct WithExprIf {
	expr: syn::Expr,
	if_:  Option<syn::Expr>,
}

impl WithExprIf {
	/// Maps the expressions in this and-with
	pub fn map(&self, mut f: impl FnMut(&syn::Expr) -> syn::Expr) -> Self {
		Self { expr: f(&self.expr), if_: self.if_.clone(), }
	}

	/// Evaluates this and-with.
	///
	/// If this contains an `if` condition, then `else_expr` will be used
	/// when that fails.
	pub fn eval(&self, else_expr: Option<syn::Expr>) -> syn::Expr {
		let Self {
			expr,
			if_
		} = self;
		match if_ {
			Some(cond) => match else_expr {
				Some(else_expr) => parse_quote! {
					if #cond { #expr }
					else { #else_expr }
				},
				None => parse_quote! { if #cond { #expr } },
			},
			None => parse_quote! { #expr },
		}
	}
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(format))]
struct VariantFieldAttrs {
	#[as_ref]
	_ident: Option<syn::Ident>,
	#[as_ref]
	ty:     syn::Type,
}

#[derive(Debug, darling::FromVariant, derive_more::AsRef)]
#[darling(attributes(format))]
struct VariantAttrs {
	#[as_ref]
	ident:        syn::Ident,
	#[as_ref]
	fields:       darling::ast::Fields<VariantFieldAttrs>,

	#[darling(default)]
	indent:       Option<Indent>,

	with:         Option<syn::Expr>,

	#[darling(default)]
	prefix_ws:    Option<WithExprIf>,

	#[darling(multiple)]
	before_with:  Vec<WithExprIf>,

	#[darling(multiple)]
	with_tag:     Vec<WithTag>,

	#[darling(default)]
	without_tags: bool,

	args:         Option<syn::Expr>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(format))]
struct FieldAttrs {
	#[as_ref]
	ident:        Option<syn::Ident>,
	#[as_ref]
	ty:           syn::Type,

	#[darling(default)]
	str:          bool,

	#[darling(default)]
	indent:       Option<Indent>,

	with:         Option<syn::Expr>,
	with_self:    Option<syn::Expr>,

	#[darling(default)]
	prefix_ws:    Option<WithExprIf>,

	#[darling(multiple)]
	before_with:  Vec<WithExprIf>,

	#[darling(multiple)]
	with_tag:     Vec<WithTag>,

	#[darling(default)]
	without_tags: bool,

	args:         Option<syn::Expr>,
}

#[derive(Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(format))]
struct Attrs {
	#[as_ref]
	ident:        syn::Ident,
	#[as_ref]
	generics:     syn::Generics,
	#[as_ref]
	data:         darling::ast::Data<VariantAttrs, FieldAttrs>,

	indent:       Option<Indent>,

	#[darling(default)]
	no_prefix_ws: bool,

	#[darling(multiple)]
	before_with:  Vec<WithExprIf>,

	#[darling(multiple)]
	with_tag:     Vec<WithTag>,

	#[darling(default)]
	without_tags: bool,

	args:         Option<ArgsTy>,

	// TODO: Don't require the `where` token here.
	where_format: Option<syn::WhereClause>,
}

pub fn derive(input: proc_macro::TokenStream) -> Result<proc_macro::TokenStream, AppError> {
	let input = syn::parse::<syn::DeriveInput>(input)
		.context("Unable to parse input")?;

	let attrs = Attrs::from_derive_input(&input)
		.context("Unable to parse attributes")?;
	let item_ident = &attrs.ident;

	let format = match &attrs.data {
		darling::ast::Data::Enum(variants) => self::derive_enum(variants)?,
		darling::ast::Data::Struct(fields) => self::derive_struct(&attrs, fields)?,
	};

	let format = self::derive_format(parse_quote! { self }, None, None, true, &None, &None, format, &attrs.before_with, &attrs.with_tag, attrs.without_tags, Args::Skip, &attrs.indent,)?;

	let prefix_ws_ty: syn::Type = match attrs.no_prefix_ws {
		true => parse_quote! { () },
		false => parse_quote! { rustidy_format::WhitespaceConfig },
	};

	let args_ty = match &attrs.args {
		Some(args) => args.ty.clone(),
		None => parse_quote! { () },
	};

	let impl_generics = match attrs.where_format {
		Some(where_) => {
			let mut generics = attrs.generics.clone();
			generics.where_clause = Some(where_);
			generics
		},
		None => {
			let generics = attrs.generics.clone();
			match &attrs.data {
				darling::ast::Data::Enum(variants) => util::with_enum_bounds(generics, variants, |variant, field| {
					let ty = &field.ty;
					match variant.args.is_some() {
						true => parse_quote! { #ty: rustidy_format::Formattable },
						false => parse_quote! { #ty: rustidy_format::Format<#prefix_ws_ty, ()> },
					}
				}),
				darling::ast::Data::Struct(fields) => util::with_struct_bounds(generics, &fields.fields, |field| {
					let ty = &field.ty;
					match field.args.is_some() {
						true => parse_quote! { #ty: rustidy_format::Formattable },
						false => parse_quote! { #ty: rustidy_format::Format<#prefix_ws_ty, ()> },
					}
				}),
			}
		},
	};

	let (_, ty_generics, impl_where_clause) = impl_generics.split_for_impl();
	let (impl_generics, ..) = {
		super let mut impl_generics = impl_generics.clone();
		if let Some(args) = &attrs.args {
			for generic in &args.generics {
				impl_generics
					.params
					.push(syn::GenericParam::Type(generic.clone()));
			}
		}
		impl_generics.split_for_impl()
	};

	let output = quote! {
		#[automatically_derived]
		impl #impl_generics rustidy_format::Format<#prefix_ws_ty, #args_ty> for #item_ident #ty_generics #impl_where_clause {
			#[coverage(on)]
			fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: #prefix_ws_ty, args: #args_ty) -> rustidy_format::FormatOutput {
				#format
			}
		}
	};

	Ok(output.into())
}

fn derive_enum(variants: &[VariantAttrs]) -> Result<syn::Expr, AppError> {
	let format_variants = variants
		.iter()
		.map(|variant| {
			let variant_ident = &variant.ident;

			let prefix_ws = variant
				.prefix_ws
				.as_ref()
				.map(|prefix_ws| prefix_ws
					.eval(Some(parse_quote! { prefix_ws })));

			let format = parse_quote! { rustidy_format::Format::format(value, ctx, prefix_ws, args) };
			let format = self::derive_format(parse_quote! { value }, prefix_ws, None, true, &variant.with, &None, format, &variant.before_with, &variant.with_tag, variant.without_tags, Args::Set(variant.args.clone()), &variant.indent)?;

			Ok(parse_quote! {
				Self::#variant_ident(ref mut value) => #format,
			})
		})
		.collect::<Result<Vec<syn::Arm>, AppError>>()?;

	Ok(parse_quote! { match *self { #( #format_variants )* } })
}

fn derive_struct(attrs: &Attrs, fields: &darling::ast::Fields<FieldAttrs>) -> Result<syn::Expr, AppError> {
	let format_fields = fields
		.iter()
		.enumerate()
		.map(|(field_idx, field)| self::derive_struct_field(attrs, field_idx, field))
		.collect::<Result<Vec<_>, _>>()?;

	let assert_prefix_ws: Option<syn::Expr> = (!attrs
		.no_prefix_ws)
		.then(|| parse_quote! {
			if !output.is_empty && !output.has_prefix_ws() {
				tracing::warn!("Non-empty type did not use prefix whitespace: {}", std::any::type_name::<Self>())
			}
		});

	Ok(parse_quote! {{
		let mut output = rustidy_format::FormatOutput::default();
		let mut has_prefix_ws = true;
		#( #format_fields; )*

		#assert_prefix_ws;
		output
	}})
}

fn derive_struct_field(attrs: &Attrs, field_idx: usize, field: &FieldAttrs) -> Result<syn::Expr, AppError> {
	let field_ident = util::field_member_access(field_idx, field);

	let prefix_ws = match &field.prefix_ws {
		Some(prefix_ws) => Some(prefix_ws
			.map(|prefix_ws| match attrs.no_prefix_ws {
				true => prefix_ws.clone(),
				false => parse_quote! {{
					if has_prefix_ws {
						tracing::warn!(
							"Overwriting prefix whitespace of {}::{}",
							std::any::type_name::<Self>(),
							stringify!(#field_ident)
						);
					}

					#prefix_ws
				}},
			})
			.eval(Some(parse_quote! { prefix_ws })),),
		None => match field.str {
			true => None,
			false => match attrs.no_prefix_ws {
				true => None,
				false => Some(parse_quote! { match has_prefix_ws {
					true => prefix_ws,
					false => {
						tracing::warn!(
							"Missing prefix whitespace for {}::{}",
							std::any::type_name::<Self>(),
							stringify!(#field_ident)
						);

						<rustidy_util::Whitespace as rustidy_format::WhitespaceFormat>::PRESERVE
					},
				}})
			},
		},
	};

	let format = match field.str {
		true => parse_quote! { <rustidy_util::AstStr as rustidy_format::AstStrFormat>::format_output(&mut self.#field_ident, ctx) },
		false => parse_quote! { rustidy_format::Format::format(&mut self.#field_ident, ctx, prefix_ws, args) },
	};

	let after_format = parse_quote! {
		if has_prefix_ws && output.has_prefix_ws() {
			has_prefix_ws = false;
		}
	};

	self::derive_format(parse_quote! { &mut self.#field_ident }, prefix_ws, Some(after_format), false, &field.with, &field.with_self, format, &field.before_with, &field.with_tag, field.without_tags, Args::Set(field.args.clone()), &field.indent,)
}

enum Args {
	Skip,
	Set(Option<syn::Expr>),
}

#[expect(
	clippy::ref_option,
	clippy::needless_pass_by_value,
	reason = "This signature is more ergonomic"
)]
#[expect(clippy::too_many_arguments, reason = "TODO")]
fn derive_format(value: syn::Expr, prefix_ws: Option<syn::Expr>, after_format: Option<syn::Expr>, return_output: bool, with: &Option<syn::Expr>, with_self: &Option<syn::Expr>, default: syn::Expr, before_with: &[WithExprIf], with_tag: &[WithTag], without_tags: bool, args: Args, indent: &Option<Indent>,) -> Result<syn::Expr, AppError> {
	// TODO: Document the order in which we parse all attributes, since
	//       it's not in declaration order (although maybe it should?).
	ensure!(with.is_none() || with_self.is_none(), "Cannot specify both `#[format(with)]` and `#[format(with_self)]`");
	let format = match &with {
		Some(with) => parse_quote! { (#with)(#value, ctx, prefix_ws, args) },
		None => match &with_self {
			Some(with) => parse_quote! { (#with)(self, ctx, prefix_ws, args) },
			None => default,
		},
	};

	let format = match return_output {
		true => format,
		false => parse_quote! { output.append(#format) },
	};
	let format = match args {
		Args::Skip => format,
		Args::Set(args) => {
			let args = args.unwrap_or_else(|| parse_quote! { () });
			parse_quote! {{
				let args = #args;
				#format
			}}
		},
	};
	let format = match prefix_ws {
		Some(prefix_ws) => parse_quote! {{
			let prefix_ws = #prefix_ws;
			#format
		}},
		None => format,
	};

	let mut format = match without_tags {
		true => parse_quote! { ctx.without_tags(|ctx| #format) },
		false => format,
	};
	for WithTag {
		tag,
		if_
	} in with_tag {
		let cond = if_
			.clone()
			.unwrap_or_else(|| parse_quote! { true });
		format = parse_quote! {
			match #cond {
				true => ctx.with_tag(#tag, |ctx| #format),
				false => #format,
			}
		}
	}

	let format = match indent {
		Some(Indent {
			if_has_tag
		}) => match if_has_tag {
			Some(cond) => parse_quote! { ctx.with_indent_if(ctx.has_tag(#cond), |ctx| #format) },
			None => parse_quote! { ctx.with_indent(|ctx| #format) },
		},
		None => format,
	};

	let before_with = before_with
		.iter()
		.map(|before_with| before_with
			.map(|expr| parse_quote! { (#expr)(#value, ctx) })
			.eval(None));
	let format = match before_with.is_empty() {
		true => format,
		false => parse_quote! {{
			#( #before_with )*;
			#format
		}},
	};

	let format = match after_format {
		Some(after_format) => match return_output {
			true => parse_quote! {{
				let output = #format;
				#after_format;
				output
			}},
			false => parse_quote! {{
				#format;
				#after_format;
			}},
		},
		None => format,
	};

	Ok(format)
}
