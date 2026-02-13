//! `derive(Format)`

// Imports
use {
	crate::util,
	app_error::{AppError, Context, app_error, bail},
	core::{iter, ops::Bound},
	darling::FromDeriveInput,
	itertools::Itertools,
	quote::quote,
	syn::parse_quote,
};

#[derive(Debug, darling::FromMeta)]
// TODO: Something better than this...
#[darling(from_expr = |expr| Ok(Self { ty: parse_quote! { #expr }, generics: vec![] }))]
struct ArgsTy {
	ty: syn::Type,

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
#[darling(from_expr = |expr| Ok(Self { tag: expr.clone(), cond: None, skip_if_has_tag: false }))]
struct WithTag {
	tag:             syn::Expr,
	#[darling(rename = "if")]
	cond:            Option<syn::Expr>,
	skip_if_has_tag: bool,
}

#[derive(Clone, Debug, darling::FromMeta)]
#[darling(from_expr = |expr| Ok(Self { expr: expr.clone(), cond: None, else_expr: None }))]
struct AndWith {
	expr:      syn::Expr,
	#[darling(rename = "if")]
	cond:      Option<syn::Expr>,
	#[darling(rename = "else")]
	else_expr: Option<syn::Expr>,
}

impl AndWith {
	/// Maps the expressions in this and-with
	pub fn map(&self, mut f: impl FnMut(&syn::Expr) -> syn::Expr) -> Self {
		Self {
			expr:      f(&self.expr),
			cond:      self.cond.clone(),
			else_expr: self.else_expr.as_ref().map(f),
		}
	}

	/// Sets the else expression in this and-with
	pub fn with_else(&self, else_expr: syn::Expr) -> Self {
		Self {
			expr:      self.expr.clone(),
			cond:      self.cond.clone(),
			else_expr: Some(else_expr),
		}
	}

	/// Evaluates this and-with
	pub fn eval(&self) -> syn::Expr {
		let Self { expr, cond, else_expr } = self;
		match cond {
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

#[derive(Debug, darling::FromMeta)]
struct AndWithWrapper {
	fields:   syn::Expr,
	#[darling(flatten)]
	and_with: AndWith,
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
	ident:  syn::Ident,
	#[as_ref]
	fields: darling::ast::Fields<VariantFieldAttrs>,

	#[darling(default)]
	indent: Option<Indent>,

	with: Option<syn::Expr>,

	#[darling(default)]
	prefix_ws: Option<AndWith>,

	#[darling(multiple)]
	before_with: Vec<AndWith>,

	#[darling(multiple)]
	and_with: Vec<AndWith>,

	#[darling(multiple)]
	with_tag: Vec<WithTag>,

	#[darling(default)]
	without_tags: bool,

	args: Option<syn::Expr>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(format))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,

	#[darling(default)]
	indent: Option<Indent>,

	with: Option<syn::Expr>,

	#[darling(default)]
	prefix_ws: Option<AndWith>,

	#[darling(multiple)]
	before_with: Vec<AndWith>,

	#[darling(multiple)]
	and_with: Vec<AndWith>,

	#[darling(multiple)]
	with_tag: Vec<WithTag>,

	#[darling(default)]
	without_tags: bool,

	args: Option<syn::Expr>,

	#[darling(default)]
	whitespace: bool,

	#[darling(default)]
	str: bool,
}

#[derive(Debug, darling::FromDeriveInput, derive_more::AsRef)]
#[darling(attributes(format))]
struct Attrs {
	#[as_ref]
	ident:    syn::Ident,
	#[as_ref]
	generics: syn::Generics,
	#[as_ref]
	data:     darling::ast::Data<VariantAttrs, FieldAttrs>,

	#[darling(default)]
	indent: Option<Indent>,

	with: Option<syn::Expr>,

	#[darling(multiple)]
	before_with: Vec<AndWith>,

	#[darling(multiple)]
	and_with: Vec<AndWith>,

	#[darling(multiple)]
	and_with_wrapper: Vec<AndWithWrapper>,

	#[darling(multiple)]
	with_tag: Vec<WithTag>,

	#[darling(default)]
	without_tags: bool,

	args: Option<ArgsTy>,

	// TODO: Don't require the `where` token here.
	where_format: Option<syn::WhereClause>,

	// TODO: Don't require the `where` token here.
	#[darling(multiple)]
	with_where_format: Vec<syn::WhereClause>,
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

	let Impls { with_strings, format } = impls;

	let format = self::derive_format(
		parse_quote! { self },
		|expr| expr,
		None,
		&attrs.with,
		format,
		&attrs.before_with,
		&attrs.and_with,
		&attrs.with_tag,
		attrs.without_tags,
		&None,
		&attrs.indent,
	);

	let format_and_with_wrapper = attrs
		.and_with_wrapper
		.iter()
		.map(|and_with_wrapper| {
			let darling::ast::Data::Struct(fields) = &attrs.data else {
				bail!("`#[format(and_with_wrapper(...))]` may only be used for structs");
			};

			self::derive_and_with_wrapper(fields, and_with_wrapper)
		})
		.collect::<Result<Vec<_>, AppError>>()?;

	let format: syn::Expr = parse_quote! {{
		#format;
		#( #format_and_with_wrapper; )*
	}};

	let formattable_impl = {
		let impl_generics = util::with_bounds(&attrs, |ty| parse_quote! { #ty: rustidy_format::Formattable });
		let (impl_generics, ty_generics, impl_where_clause) = impl_generics.split_for_impl();
		quote! {
			#[automatically_derived]
			impl #impl_generics rustidy_format::Formattable for #item_ident #ty_generics #impl_where_clause {
				fn with_strings<WITH_STRINGS_WS_O>(
					&mut self,
					ctx: &mut rustidy_format::Context,
					mut exclude_prefix_ws: bool,
					f: &mut impl FnMut(&mut rustidy_util::AstStr, &mut rustidy_format::Context) -> std::ops::ControlFlow<WITH_STRINGS_WS_O>,
				) -> std::ops::ControlFlow<WITH_STRINGS_WS_O> {
					#with_strings
				}
			}
		}
	};

	let args_ty = match &attrs.args {
		Some(args) => args.ty.clone(),
		None => parse_quote! { () },
	};

	let format_impl = {
		let mut impl_generics = match attrs.where_format {
			Some(where_) => {
				let mut generics = attrs.generics.clone();
				generics.where_clause = Some(where_);
				generics
			},
			None => match attrs.with.is_some() {
				true => attrs.generics.clone(),
				false => {
					let generics = attrs.generics.clone();
					match &attrs.data {
						darling::ast::Data::Enum(variants) =>
							util::with_enum_bounds(generics, variants, |variant, field| {
								let ty = &field.ty;
								match variant.args.is_some() {
									true => parse_quote! { #ty: },
									false => parse_quote! { #ty: rustidy_format::Format<#args_ty> },
								}
							}),
						darling::ast::Data::Struct(fields) =>
							util::with_struct_bounds(generics, &fields.fields, |field| {
								let ty = &field.ty;
								match field.args.is_some() {
									true => parse_quote! { #ty: },
									false => parse_quote! { #ty: rustidy_format::Format<#args_ty> },
								}
							}),
					}
				},
			},
		};
		impl_generics
			.make_where_clause()
			.predicates
			.extend(attrs.with_where_format.into_iter().flat_map(|clause| clause.predicates));

		let (_, ty_generics, impl_where_clause) = impl_generics.split_for_impl();
		let (impl_generics, ..) = {
			super let mut impl_generics = impl_generics.clone();
			if let Some(args) = &attrs.args {
				for generic in &args.generics {
					impl_generics.params.push(syn::GenericParam::Type(generic.clone()));
				}
			}
			impl_generics.split_for_impl()
		};

		quote! {
			#[automatically_derived]
			impl #impl_generics rustidy_format::Format<#args_ty> for #item_ident #ty_generics #impl_where_clause {
				#[coverage(on)]
				fn format(&mut self, ctx: &mut rustidy_format::Context, prefix_ws: &mut impl rustidy_format::FormatFn<rustidy_util::Whitespace>, args: &mut #args_ty) {
					#format;
				}
			}
		}
	};

	let output = quote! {
		#formattable_impl
		#format_impl
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

			let prefix_ws: syn::Expr = match &variant.prefix_ws {
				Some(prefix_ws) => match prefix_ws.cond.is_some() && prefix_ws.else_expr.is_none() {
					true => prefix_ws.with_else(parse_quote! { *prefix_ws }).eval(),
					false => prefix_ws.eval(),
				},
				None => parse_quote! { *prefix_ws },
			};

			let format = parse_quote! { match #prefix_ws {
				ref mut prefix_ws => rustidy_format::Format::format(value, ctx, prefix_ws, args)
			}};

			let format = self::derive_format(
				parse_quote! { value },
				|expr| parse_quote! { match #prefix_ws {
					ref mut prefix_ws => #expr,
				}},
				None,
				&variant.with,
				format,
				&variant.before_with,
				&variant.and_with,
				&variant.with_tag,
				variant.without_tags,
				&variant.args,
				&variant.indent,
			);
			let format = parse_quote! {
				Self::#variant_ident(ref mut value) => #format,
			};

			Impls {
				with_strings,
				format,
			}
		})
		.collect::<Impls<Vec<syn::Arm>, Vec<syn::Arm>>>();


	let Impls { with_strings, format } = variant_impls;
	let with_strings = parse_quote! { match *self { #( #with_strings )* } };
	let format = parse_quote! { match *self { #( #format )* } };

	Impls { with_strings, format }
}

fn derive_struct(fields: &darling::ast::Fields<FieldAttrs>) -> Impls<syn::Expr, syn::Expr> {
	let Impls { with_strings, format } = fields
		.iter()
		.enumerate()
		.map(|(field_idx, field)| self::derive_struct_field(field_idx, field))
		.collect::<Impls<Vec<_>, Vec<_>>>();

	let with_strings = parse_quote! {{ #( #with_strings; )* std::ops::ControlFlow::Continue(()) }};
	let format = parse_quote! {{
		let mut has_prefix_ws = true;
		#( #format; )*
	}};

	Impls { with_strings, format }
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

	let with_prefix_ws = |expr| match field.str {
		true => expr,
		false => match &field.prefix_ws {
			Some(prefix_ws) => {
				let prefix_ws = match prefix_ws.cond.is_some() && prefix_ws.else_expr.is_none() {
					true => prefix_ws.with_else(parse_quote! { *prefix_ws }),
					false => prefix_ws.clone(),
				};

				prefix_ws
					.map(|prefix_ws| {
						parse_quote! { match #prefix_ws {
							ref mut prefix_ws => #expr,
						}}
					})
					.eval()
			},
			None => parse_quote! { match has_prefix_ws {
				true => #expr,
				// TODO: Ideally here we'd panic once we ensure
				//       the caller can always provide a prefix whitespace.
				false => {
					let prefix_ws = &mut <rustidy_util::Whitespace as rustidy_format::WhitespaceFormat>::preserve;
					#expr;
				},
			}},
		},
	};

	let format = match field.str {
		true => parse_quote! { () },
		false => parse_quote! { rustidy_format::Format::format(&mut self.#field_ident, ctx, prefix_ws, args) },
	};

	let after_format = match field.whitespace {
		true => parse_quote! { has_prefix_ws = false },
		false => parse_quote! {
			// TODO: Make `format` return this so we don't have to recurse back into the type
			if has_prefix_ws && !rustidy_format::Formattable::is_empty(&mut self.#field_ident, ctx, false) {
				has_prefix_ws = false;
			}
		},
	};

	let format = self::derive_format(
		parse_quote! { &mut self.#field_ident },
		with_prefix_ws,
		Some(after_format),
		&field.with,
		format,
		&field.before_with,
		&field.and_with,
		&field.with_tag,
		field.without_tags,
		&field.args,
		&field.indent,
	);

	Impls { with_strings, format }
}

#[expect(
	clippy::ref_option,
	clippy::needless_pass_by_value,
	reason = "This signature is more ergonomic"
)]
#[expect(clippy::too_many_arguments, reason = "TODO")]
fn derive_format(
	value: syn::Expr,
	with_prefix_ws: impl Fn(syn::Expr) -> syn::Expr,
	after_format: Option<syn::Expr>,
	with: &Option<syn::Expr>,
	default: syn::Expr,
	before_with: &[AndWith],
	and_with: &[AndWith],
	with_tag: &[WithTag],
	without_tags: bool,
	args: &Option<syn::Expr>,
	indent: &Option<Indent>,
) -> syn::Expr {
	// TODO: Document the order in which we parse all attributes, since
	//       it's not in declaration order (although maybe it should?).

	let format: syn::Expr = match &with {
		Some(with) => parse_quote! { (#with)(#value, ctx, prefix_ws, args) },
		None => default,
	};
	let format = match args {
		Some(args) => parse_quote! {{
			let mut args = &mut #args;
			#format;
		}},
		None => format,
	};
	let format = with_prefix_ws(format);

	let and_with = and_with
		.iter()
		.map(|and_with| and_with.map(|expr| parse_quote! { (#expr)(#value, ctx) }).eval());
	let format: syn::Expr = parse_quote! {{
		#format;
		#( #and_with; )*
	}};


	let mut format = match without_tags {
		true => parse_quote! { ctx.without_tags(|ctx| #format) },
		false => format,
	};
	for WithTag {
		tag,
		cond,
		skip_if_has_tag,
	} in with_tag
	{
		let cond = cond.clone().unwrap_or_else(|| parse_quote! { true });
		let cond: syn::Expr = match skip_if_has_tag {
			true => parse_quote! { ctx.has_tag(#tag) || (#cond) },
			false => parse_quote! { #cond },
		};
		format = parse_quote! {
			match #cond {
				true => ctx.with_tag(#tag, |ctx| #format),
				false => #format,
			}
		}
	}

	let format = match indent {
		Some(Indent { if_has_tag }) => match if_has_tag {
			Some(cond) => parse_quote! { ctx.with_indent_if(ctx.has_tag(#cond), |ctx| #format) },
			None => parse_quote! { ctx.with_indent(|ctx| #format) },
		},
		None => format,
	};

	let before_with = before_with
		.iter()
		.map(|and_with| and_with.map(|expr| parse_quote! { (#expr)(#value, ctx) }).eval());
	parse_quote! {{
		#( #before_with; )*
		#format;

		#after_format;
	}}
}

// TODO: Remove this
fn derive_and_with_wrapper(
	fields: &darling::ast::Fields<FieldAttrs>,
	and_with_wrapper: &AndWithWrapper,
) -> Result<syn::Expr, AppError> {
	fn parse_expr(expr: &syn::Expr) -> Result<syn::Member, AppError> {
		match expr {
			syn::Expr::Path(field) if let Some(ident) = field.path.get_ident() => Ok(syn::Member::Named(ident.clone())),
			syn::Expr::Lit(syn::ExprLit {
				lit: syn::Lit::Int(lit),
				..
			}) => Ok(syn::Member::Unnamed(syn::Index {
				index: lit.base10_parse().context("Unable to parse integer literal")?,
				span:  lit.span(),
			})),
			_ => bail!("Expected an identifier or integer literal"),
		}
	}

	fn find_field<'a>(
		fields: &'a darling::ast::Fields<FieldAttrs>,
		member: &syn::Member,
	) -> Result<(usize, &'a FieldAttrs), AppError> {
		fields
			.fields
			.iter()
			.enumerate()
			.try_find(|&(field_idx, field)| match &field.ident {
				Some(field_ident) => match member {
					syn::Member::Named(ident) => Ok::<_, AppError>(field_ident == ident),
					syn::Member::Unnamed(_) => bail!("Cannot use integer literal for named structs"),
				},
				None => match member {
					syn::Member::Named(_) => bail!("Cannot use identifier for unnamed structs"),
					syn::Member::Unnamed(index) => Ok(index.index as usize == field_idx),
				},
			})?
			.ok_or_else(|| match member {
				syn::Member::Named(ident) => app_error!("Unknown field identifier: {ident}"),
				syn::Member::Unnamed(index) => app_error!("Unknown field index: {}", index.index),
			})
	}

	let wrapper_field_idents = match &and_with_wrapper.fields {
		syn::Expr::Array(expr) => expr
			.elems
			.iter()
			.map(parse_expr)
			.collect::<Result<Vec<_>, AppError>>()?,
		syn::Expr::Range(expr) => {
			fn find_expr_idx(fields: &darling::ast::Fields<FieldAttrs>, expr: &syn::Expr) -> Result<usize, AppError> {
				find_field(fields, &parse_expr(expr)?).map(|(idx, _)| idx)
			}

			let start_idx = match &expr.start {
				Some(start) => Some(find_expr_idx(fields, start)?),
				None => None,
			};
			let start = match start_idx {
				Some(start) => Bound::Included(start),
				None => Bound::Unbounded,
			};

			let end = match &expr.end {
				Some(end) => match expr.limits {
					syn::RangeLimits::HalfOpen(_) => Bound::Excluded(find_expr_idx(fields, end)?),
					syn::RangeLimits::Closed(_) => Bound::Included(find_expr_idx(fields, end)?),
				},
				None => Bound::Unbounded,
			};

			let fields = &fields.fields[(start, end)];

			iter::zip(start_idx.unwrap_or(0).., fields)
				.map(|(field_idx, field)| util::field_member_access(field_idx, field))
				.collect()
		},
		_ => bail!("`#[and_with_wrapper(fields = ...)]` expects either an array of fields or a range"),
	};

	let wrapper_field_tys = wrapper_field_idents
		.iter()
		.map(|wrapper_field_ident| find_field(fields, wrapper_field_ident))
		.map_ok(|(_, field)| &field.ty)
		.collect::<Result<Vec<_>, AppError>>()?;

	let expr = and_with_wrapper
		.and_with
		.map(|expr| {
			let wrapper: syn::ItemStruct = match fields.style {
				darling::ast::Style::Tuple => parse_quote! {
					#[derive(Debug, rustidy_format::Format)]
					pub struct Wrapper<'a>(
						#(
							&'a mut #wrapper_field_tys,
						)*
					);
				},
				darling::ast::Style::Struct => parse_quote! {
					#[derive(Debug, rustidy_format::Format)]
					pub struct Wrapper<'a> {
						#(
							#wrapper_field_idents: &'a mut #wrapper_field_tys,
						)*
					}
				},
				darling::ast::Style::Unit => parse_quote! {
					#[derive(Debug, rustidy_format::Format)]
					pub struct Wrapper;
				},
			};

			parse_quote! {{
				#wrapper

				let mut wrapper = Wrapper {
					#(
						#wrapper_field_idents: &mut self.#wrapper_field_idents,
					)*
				};

				(#expr)(&mut wrapper, ctx);
			}}
		})
		.eval();

	Ok(expr)
}

#[derive(Default, Debug)]
struct Impls<WithStrings, Format> {
	with_strings: WithStrings,
	format:       Format,
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
			output.format.extend_one(impls.format);
		}

		output
	}
}
