//! `derive(Format)`

// Imports
use {
	crate::util,
	app_error::{AppError, Context},
	core::{iter, ops::Bound},
	darling::FromDeriveInput,
	quote::quote,
	syn::parse_quote,
};

#[derive(Debug, darling::FromMeta)]
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
	indent: bool,

	with: Option<syn::Expr>,

	#[darling(multiple)]
	and_with: Vec<AndWith>,
}

#[derive(Debug, darling::FromField, derive_more::AsRef)]
#[darling(attributes(format))]
struct FieldAttrs {
	#[as_ref]
	ident: Option<syn::Ident>,
	#[as_ref]
	ty:    syn::Type,

	#[darling(default)]
	indent: bool,

	#[darling(default)]
	str: bool,

	#[darling(default)]
	whitespace: bool,

	with: Option<syn::Expr>,

	#[darling(multiple)]
	and_with: Vec<AndWith>,
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
	indent: bool,

	with: Option<syn::Expr>,

	#[darling(multiple)]
	and_with: Vec<AndWith>,

	#[darling(multiple)]
	and_with_wrapper: Vec<AndWithWrapper>,
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

	let impl_generics = util::with_bounds(&attrs, |ty| parse_quote! { #ty: crate::format::Format });
	let (impl_generics, ty_generics, fmt_where_clause) = impl_generics.split_for_impl();

	let Impls {
		range,
		len,
		format,
		prefix_ws,
	} = impls;

	let format = self::derive_format(
		parse_quote! { self },
		&attrs.with,
		format,
		&attrs.and_with,
		attrs.indent,
	);

	let format_and_with_wrapper = attrs.and_with_wrapper.iter().map(|and_with_wrapper| {
		let darling::ast::Data::Struct(fields) = &attrs.data else {
			panic!("`#[format(and_with_wrapper(...))]` may only be used for structs");
		};

		self::derive_and_with_wrapper(fields, and_with_wrapper)
	});

	let format: syn::Expr = parse_quote! {{
		#format;
		#( #format_and_with_wrapper; )*
	}};

	let output = quote! {
		#[automatically_derived]
		impl #impl_generics crate::format::Format for #item_ident #ty_generics #fmt_where_clause {
			fn range(&mut self, ctx: &mut crate::format::Context) -> Option<crate::parser::ParserRange> {
				#range
			}

			fn len(&mut self, ctx: &mut crate::format::Context) -> usize {
				#len
			}

			#[coverage(on)]
			fn format(&mut self, ctx: &mut crate::format::Context) {
				#format;
			}

			#[allow(unreachable_code)]
			fn prefix_ws(&mut self, ctx: &mut crate::format::Context) -> Option<&mut crate::ast::whitespace::Whitespace> {
				#prefix_ws
			}
		}
	};

	Ok(output.into())
}

fn derive_enum(variants: &[VariantAttrs]) -> Impls<syn::Expr, syn::Expr, syn::Expr, syn::Expr> {
	let variant_impls = variants
		.iter()
		.map(|variant| {
			let variant_ident = &variant.ident;
			let range =
				parse_quote! { Self::#variant_ident(ref mut value) => crate::format::Format::range(value, ctx), };
			let len = parse_quote! { Self::#variant_ident(ref mut value) => crate::format::Format::len(value, ctx), };

			let format = self::derive_format(
				parse_quote! { value },
				&variant.with,
				parse_quote! { crate::format::Format::format(value, ctx) },
				&variant.and_with,
				variant.indent,
			);
			let format = parse_quote! {
				Self::#variant_ident(ref mut value) => #format,
			};

			let prefix_ws = parse_quote! { Self::#variant_ident(ref mut value) => value.prefix_ws(ctx), };

			Impls {
				range,
				len,
				format,
				prefix_ws,
			}
		})
		.collect::<Impls<Vec<syn::Arm>, Vec<syn::Arm>, Vec<syn::Arm>, Vec<syn::Arm>>>();


	let Impls {
		range,
		len,
		format,
		prefix_ws,
	} = variant_impls;
	let range = parse_quote! { match *self { #( #range )* } };
	let len = parse_quote! { match *self { #( #len )* } };
	let format = parse_quote! { match *self { #( #format )* } };
	let prefix_ws = parse_quote! { match *self { #( #prefix_ws )* } };

	Impls {
		range,
		len,
		format,
		prefix_ws,
	}
}

fn derive_struct(fields: &darling::ast::Fields<FieldAttrs>) -> Impls<syn::Expr, syn::Expr, syn::Expr, syn::Expr> {
	let Impls {
		range,
		len,
		format,
		prefix_ws: (),
	} = fields
		.iter()
		.enumerate()
		.map(|(field_idx, field)| self::derive_struct_field(field_idx, field))
		.collect::<Impls<Vec<_>, Vec<_>, Vec<_>, ()>>();

	let range = parse_quote! {{
		let mut compute_range = crate::format::ComputeRange::default();
		#( #range; )*
		compute_range.finish()
	}};
	let len = parse_quote! { 0 #( + #len )* };
	let format = parse_quote! {{ #( #format; )* }};

	let prefix_ws_fields = fields.iter().enumerate().map(|(field_idx, field)| -> syn::Expr {
		let field_ident = util::field_member_access(field_idx, field);

		if field.str {
			return parse_quote! { return None };
		}

		if field.whitespace {
			return parse_quote! { return Some(&mut self.#field_ident) };
		}

		parse_quote! {{
			// TODO: Once polonius comes around, move this down
			let is_empty = crate::format::Format::is_empty(&mut self.#field_ident, ctx);

			// If we got the whitespace, return it
			if let Some(whitespace) = crate::format::Format::prefix_ws(&mut self.#field_ident, ctx) {
				return Some(whitespace);
			}

			// Otherwise, if this field had any length, we have no more fields
			// to check and we can return
			if !is_empty {
				return None;
			}
		}}
	});
	let prefix_ws = parse_quote! {{
		#( #prefix_ws_fields; )*

		None
	}};

	Impls {
		range,
		len,
		format,
		prefix_ws,
	}
}

fn derive_struct_field(field_idx: usize, field: &FieldAttrs) -> Impls<syn::Expr, syn::Expr, syn::Expr, ()> {
	let field_ident = util::field_member_access(field_idx, field);

	let range = match field.str {
		true => parse_quote! { compute_range.add_str(&mut self.#field_ident, ctx) },
		false => parse_quote! { compute_range.add(&mut self.#field_ident, ctx) },
	};
	let len = match field.str {
		true => parse_quote! { ctx.parser().str(&self.#field_ident).len() },
		false => parse_quote! { crate::format::Format::len(&mut self.#field_ident, ctx) },
	};

	let format = self::derive_format(
		parse_quote! { &mut self.#field_ident },
		&field.with,
		match field.str || field.whitespace {
			true => parse_quote! { () },
			false => parse_quote! { crate::format::Format::format(&mut self.#field_ident, ctx) },
		},
		&field.and_with,
		field.indent,
	);

	Impls {
		range,
		len,
		format,
		prefix_ws: (),
	}
}

#[expect(
	clippy::ref_option,
	clippy::needless_pass_by_value,
	reason = "This signature is more ergonomic"
)]
fn derive_format(
	value: syn::Expr,
	with: &Option<syn::Expr>,
	default: syn::Expr,
	and_with: &[AndWith],
	indent: bool,
) -> syn::Expr {
	let format: syn::Expr = match &with {
		Some(with) => parse_quote! { (#with)(#value, ctx) },
		None => default,
	};
	let and_with = and_with
		.iter()
		.map(|and_with| and_with.map(|expr| parse_quote! { (#expr)(#value, ctx) }).eval());
	let format: syn::Expr = parse_quote! {{
		#format;
		#( #and_with; )*
	}};
	match indent {
		true => parse_quote! { ctx.with_indent(|ctx| #format) },
		false => format,
	}
}

fn derive_and_with_wrapper(fields: &darling::ast::Fields<FieldAttrs>, and_with_wrapper: &AndWithWrapper) -> syn::Expr {
	fn parse_expr(expr: &syn::Expr) -> syn::Member {
		match expr {
			syn::Expr::Path(field) if let Some(ident) = field.path.get_ident() => syn::Member::Named(ident.clone()),
			syn::Expr::Lit(syn::ExprLit {
				lit: syn::Lit::Int(lit),
				..
			}) => syn::Member::Unnamed(syn::Index {
				index: lit.base10_parse().expect("Unable to parse integer literal"),
				span:  lit.span(),
			}),
			_ => panic!("Expected an identifier or integer literal"),
		}
	}

	fn find_field<'a>(fields: &'a darling::ast::Fields<FieldAttrs>, member: &syn::Member) -> (usize, &'a FieldAttrs) {
		fields
			.fields
			.iter()
			.enumerate()
			.find(|&(field_idx, field)| match &field.ident {
				Some(field_ident) => match member {
					syn::Member::Named(ident) => field_ident == ident,
					syn::Member::Unnamed(_) => panic!("Cannot use integer literal for named structs"),
				},
				None => match member {
					syn::Member::Named(_) => panic!("Cannot use identifier for unnamed structs"),
					syn::Member::Unnamed(index) => index.index as usize == field_idx,
				},
			})
			.unwrap_or_else(|| match member {
				syn::Member::Named(ident) => panic!("Unknown field identifier: {ident}"),
				syn::Member::Unnamed(index) => panic!("Unknown field index: {}", index.index),
			})
	}

	let wrapper_field_idents = match &and_with_wrapper.fields {
		syn::Expr::Array(expr) => expr.elems.iter().map(parse_expr).collect::<Vec<_>>(),
		syn::Expr::Range(expr) => {
			fn find_expr_idx(fields: &darling::ast::Fields<FieldAttrs>, expr: &syn::Expr) -> usize {
				find_field(fields, &parse_expr(expr)).0
			}

			let start_idx = expr.start.as_ref().map(|start| find_expr_idx(fields, start));
			let start = match start_idx {
				Some(start) => Bound::Included(start),
				None => Bound::Unbounded,
			};

			let end = match &expr.end {
				Some(end) => match expr.limits {
					syn::RangeLimits::HalfOpen(_) => Bound::Excluded(find_expr_idx(fields, end)),
					syn::RangeLimits::Closed(_) => Bound::Included(find_expr_idx(fields, end)),
				},
				None => Bound::Unbounded,
			};

			let fields = &fields.fields[(start, end)];

			iter::zip(start_idx.unwrap_or(0).., fields)
				.map(|(field_idx, field)| util::field_member_access(field_idx, field))
				.collect()
		},
		_ => panic!("`#[and_with_wrapper(fields = ...)]` expects either an array of fields or a range"),
	};

	let wrapper_field_tys = wrapper_field_idents
		.iter()
		.map(|wrapper_field_ident| &find_field(fields, wrapper_field_ident).1.ty)
		.collect::<Vec<_>>();

	and_with_wrapper
		.and_with
		.map(|expr| {
			let wrapper: syn::ItemStruct = match fields.style {
				darling::ast::Style::Tuple => parse_quote! {
					#[derive(Debug, crate::format::Format)]
					pub struct Wrapper<'a>(
						#(
							&'a mut #wrapper_field_tys,
						)*
					);
				},
				darling::ast::Style::Struct => parse_quote! {
					#[derive(Debug, crate::format::Format)]
					pub struct Wrapper<'a> {
						#(
							#wrapper_field_idents: &'a mut #wrapper_field_tys,
						)*
					}
				},
				darling::ast::Style::Unit => parse_quote! {
					#[derive(Debug, crate::format::Format)]
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
		.eval()
}

#[derive(Default, Debug)]
struct Impls<Range, Len, Format, PrefixWs> {
	range:     Range,
	len:       Len,
	format:    Format,
	prefix_ws: PrefixWs,
}

impl<T0, T1, T2, T3, A0, A1, A2, A3> FromIterator<Impls<A0, A1, A2, A3>> for Impls<T0, T1, T2, T3>
where
	T0: Default + Extend<A0>,
	T1: Default + Extend<A1>,
	T2: Default + Extend<A2>,
	T3: Default + Extend<A3>,
{
	fn from_iter<I: IntoIterator<Item = Impls<A0, A1, A2, A3>>>(iter: I) -> Self {
		let mut output = Self::default();
		for impls in iter {
			output.range.extend_one(impls.range);
			output.len.extend_one(impls.len);
			output.format.extend_one(impls.format);
			output.prefix_ws.extend_one(impls.prefix_ws);
		}

		output
	}
}
