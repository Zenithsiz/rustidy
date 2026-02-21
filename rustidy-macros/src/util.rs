//! Utilities

// TODO: Replace all of this with just `impl IntoIterator` + `impl AsRef<>` once we're
//       no longer using `syn`'s types.

// Imports
use proc_macro2::Span;

/// Creates new generics with an added trait bound of `path` on each variant of `item`
///
/// If the item is not generic, no bounds are added.
pub fn with_enum_bounds<V, F>(mut generics: syn::Generics, variants: &[V], create_bound: impl Fn(&V,&F) -> syn::WherePredicate,) -> syn::Generics
where
	V: AsRef<darling::ast::Fields<F>>,
{
	// If we have no generic parameters, just return them
	if generics.params.is_empty() {
		return generics;
	}

	// Add each variant's type
	let where_clause = generics.make_where_clause();
	for variant in variants {
		for field in variant.as_ref().iter() {
			where_clause
				.predicates
				.push(create_bound(variant, field));
		}
	}

	generics
}

/// Creates new generics with an added trait bound of `path` on each field of `item`
///
/// If the item is not generic, no bounds are added.
pub fn with_struct_bounds<F>(mut generics: syn::Generics, fields: &[F], create_bound: impl Fn(&F) -> syn::WherePredicate,) -> syn::Generics {
	// If we have no generic parameters, just return them
	if generics.params.is_empty() {
		return generics;
	}

	// Add each field's type
	let where_clause = generics.make_where_clause();
	for field in fields {
		where_clause.predicates.push(create_bound(field));
	}

	generics
}

/// Creates new generics with an added trait bound of `path` on each type the item holds.
///
/// If the item is not generic, no bounds are added
pub fn with_bounds<A, V, VF, F>(attrs: &A, create_bound: impl Fn(&syn::Type) -> syn::WherePredicate) -> syn::Generics
where
	A: AsRef<syn::Generics> + AsRef<darling::ast::Data<V, F>>,
	V: AsRef<darling::ast::Fields<VF>>,
	VF: AsRef<syn::Type>,
	F: AsRef<syn::Type>,
{
	let generics = <A as AsRef<syn::Generics>>::as_ref(attrs)
		.clone();
	match <A as AsRef<darling::ast::Data<V, F>>>::as_ref(attrs) {
		darling::ast::Data::Enum(variants) => self::with_enum_bounds(generics, variants, |_, field| create_bound(field.as_ref())),
		darling::ast::Data::Struct(fields) => self::with_struct_bounds(generics, &fields.fields, |field| create_bound(field.as_ref())),
	}
}


/// Gets the member access for a field
pub fn field_member_access<F: AsRef<Option<syn::Ident>>>(field_idx: usize, field: F) -> syn::Member {
	match field.as_ref() {
		Some(ident) => syn::Member::Named(ident.clone()),
		None => syn::Member::Unnamed(syn::Index {
			#[expect(clippy::cast_possible_truncation, reason = "There shouldn't be more than 2^32 fields in a struct")]
			index: field_idx as u32,
			span: Span::call_site(),
		}),
	}
}

/// Fallible `Iterator::unzip`.
#[extend::ext(name = IteratorTryUnzip)]
pub impl<A, B, E, I: IntoIterator<Item = Result<(A, B), E>>> I {
	fn try_unzip<FromA, FromB>(self) -> Result<(FromA, FromB), E>
	where
		FromA: Default + Extend<A>,
		FromB: Default + Extend<B>,
	{
		let mut a = FromA::default();
		let mut b = FromB::default();
		for item in self {
			let item = item?;
			a.extend_one(item.0);
			b.extend_one(item.1);
		}

		Ok((a, b))
	}
}

#[derive(Debug)]
pub struct Fmt {
	pub parts: Vec<syn::Expr>,
}

impl darling::FromMeta for Fmt {
	fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
		Ok(Self { parts: vec![expr.clone()], })
	}

	fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
		let parts = items
			.iter()
			.map(|meta| match meta {
				darling::ast::NestedMeta::Meta(meta) => match meta {
					syn::Meta::Path(path) => Ok(syn::Expr::Path(syn::ExprPath {
						attrs: vec![],
						qself: None,
						path: path.clone(),
					})),
					syn::Meta::List(_) => todo!("Expected a literal or path"),
					syn::Meta::NameValue(_) => todo!("Expected a literal or path"),
				},
				darling::ast::NestedMeta::Lit(lit) => Ok(syn::Expr::Lit(syn::ExprLit { attrs: vec![], lit: lit.clone(), })),
			})
			.collect::<Result<Vec<_>, darling::Error>>()?;

		Ok(Self { parts })
	}
}
