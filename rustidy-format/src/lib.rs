//! Formatting

// Features
#![feature(
	decl_macro,
	never_type,
	coverage_attribute,
	macro_metavar_expr_concat,
	trait_alias,
	iter_advance_by
)]

// Exports
pub use rustidy_macros::Format;

// Imports
use {
	crate as rustidy_format,
	core::marker::PhantomData,
	rustidy_util::{ArenaData, ArenaIdx, AstRange, AstStr, Config, Replacement, Replacements, ast_str::AstStrRepr},
};

/// Whitespace-like for formatting
// TODO: Once we move into our own crate, just rename this to `Whitespace`
pub trait WhitespaceLike: Format {
	/// Returns if this whitespace is pure
	fn is_pure(&mut self, ctx: &mut Context) -> bool;

	/// Removes this whitespace
	fn remove(&mut self, ctx: &mut Context);

	/// Sets this whitespace to a single space
	fn set_single(&mut self, ctx: &mut Context);

	/// Sets this whitespace to indentation.
	fn set_indent(&mut self, ctx: &mut Context, offset: isize, remove_if_empty: bool);
}

/// Whitespace visitor
pub trait WhitespaceVisitor {
	type Output;

	fn visit<W: WhitespaceLike>(&mut self, whitespace: &mut W, context: &mut Context) -> Self::Output;
}

/// Formattable type
pub trait Format {
	/// Returns the input range of this type
	fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange>;

	/// Returns the input length of this type
	fn input_len(&mut self, ctx: &mut Context) -> usize {
		self.input_range(ctx).map_or(0, |range| range.len())
	}

	/// Returns if the input string of this type is blank (consisting of just pure whitespace)
	fn input_is_blank(&mut self, ctx: &mut Context) -> bool {
		match self.input_range(ctx) {
			Some(range) => range.str(ctx.input).bytes().all(|ch| ch.is_ascii_whitespace()),
			None => true,
		}
	}

	/// Iterates over all output strings
	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context));

	/// Returns the output length of this type
	fn output_len(&mut self, ctx: &mut Context) -> usize {
		let mut len = 0;
		self.with_output(ctx, &mut |s, ctx| match ctx.replacements.get(s) {
			Some(replacement) => len += replacement.len(),
			None => match s.repr() {
				AstStrRepr::AstRange(range) => len += range.len(),
				AstStrRepr::String(s) => len += s.len(),
			},
		});
		len
	}

	/// Returns if the output is blank
	fn output_is_blank(&mut self, ctx: &mut Context) -> bool {
		let mut is_blank = true;
		self.with_output(ctx, &mut |s, ctx| match ctx.replacements.get(s) {
			Some(replacement) => is_blank &= replacement.is_blank(ctx.config),
			None => match s.repr() {
				AstStrRepr::AstRange(range) => is_blank &= rustidy_util::is_str_blank(range.str(ctx.input)),
				AstStrRepr::String(s) => is_blank &= rustidy_util::is_str_blank(s),
			},
		});

		is_blank
	}

	/// Returns the output length of this type without the prefix whitespace
	fn output_len_without_prefix_ws(&mut self, ctx: &mut Context) -> usize {
		let mut len = self.output_len(ctx);
		self.with_prefix_ws(ctx, &mut self::whitespace_visitor! {
			lifetimes<'a>
			capture(len: &'a mut usize = &mut len)
			|ws, ctx| -> () => **len -= ws.output_len(ctx)
		});

		len
	}

	/// Formats this type
	fn format(&mut self, ctx: &mut Context);

	/// Uses the first whitespace of this type, if any.
	///
	/// Returns if successfully used.
	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output>;

	/// Returns if the prefix whitespace is pure
	fn prefix_ws_is_pure(&mut self, ctx: &mut Context) -> bool {
		self.with_prefix_ws(
			ctx,
			&mut self::whitespace_visitor! { |ws, ctx| -> bool => ws.is_pure(ctx) },
		)
		.unwrap_or(false)
	}

	/// Remove the prefix whitespace, if any
	fn prefix_ws_remove(&mut self, ctx: &mut Context) {
		self.with_prefix_ws(
			ctx,
			&mut self::whitespace_visitor! { |ws, ctx| -> () => ws.remove(ctx) },
		);
	}

	/// Sets the prefix whitespace to a single space
	fn prefix_ws_set_single(&mut self, ctx: &mut Context) {
		self.with_prefix_ws(
			ctx,
			&mut self::whitespace_visitor! { |ws, ctx| -> () => ws.set_single(ctx) },
		);
	}

	/// Sets the prefix whitespace to an indentation
	fn prefix_ws_set_indent(&mut self, ctx: &mut Context, offset: isize, remove_if_empty: bool) {
		self.with_prefix_ws(ctx, &mut self::whitespace_visitor! {
			capture(offset: isize, remove_if_empty: bool)
			|ws, ctx| -> () => ws.set_indent(ctx, *offset, *remove_if_empty)
		});
	}

	/// Sets the prefix whitespace to the current indentation without removing if empty
	fn prefix_ws_set_cur_indent(&mut self, ctx: &mut Context) {
		self.prefix_ws_set_indent(ctx, 0, false);
	}
}

macro whitespace_visitor(
	$(
		lifetimes<
			$($generic_lifetime:lifetime),* $(,)?
		>
	)?
	$(
		capture(
			$(
				$capture:ident: $CaptureTy:ty $( = $capture_expr:expr)?
			),*

			$(,)?
		)
	)?
	| $whitespace:ident, $context:ident | -> $OutputTy:ty => $output:expr
) {{
	struct Visitor
	$(<
			$($generic_lifetime,)*
	>)?
	{
		$(
			$(
				$capture: $CaptureTy,
			)*
		)?
	}

	impl$(< $($generic_lifetime,)* >)? WhitespaceVisitor for Visitor $(< $($generic_lifetime,)* >)? {
		type Output = $OutputTy;

		fn visit<W: WhitespaceLike>(&mut self, $whitespace: &mut W, $context: &mut Context) -> Self::Output {
			$( let Self { $( $capture, )* } = self; )?
			$output
		}
	}

	Visitor { $( $( $capture $(: $capture_expr)?, )* )? }
}}

impl<T: Format> Format for &'_ mut T {
	fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange> {
		(**self).input_range(ctx)
	}

	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context)) {
		(**self).with_output(ctx, f);
	}

	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output> {
		(**self).with_prefix_ws(ctx, visitor)
	}
}

impl<T: Format> Format for Box<T> {
	fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange> {
		(**self).input_range(ctx)
	}

	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context)) {
		(**self).with_output(ctx, f);
	}

	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output> {
		(**self).with_prefix_ws(ctx, visitor)
	}
}

impl<T: Format> Format for Option<T> {
	fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange> {
		self.as_mut()?.input_range(ctx)
	}

	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context)) {
		if let Some(value) = self {
			value.with_output(ctx, f);
		}
	}

	fn format(&mut self, ctx: &mut Context) {
		if let Some(value) = self {
			value.format(ctx);
		}
	}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output> {
		match self {
			Some(value) => value.with_prefix_ws(ctx, visitor),
			_ => None,
		}
	}
}

impl<T: Format> Format for Vec<T> {
	fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange> {
		let mut compute_range = ComputeRange::default();
		compute_range.extend(self, ctx);
		compute_range.finish()
	}

	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context)) {
		for value in self {
			value.with_output(ctx, f);
		}
	}

	fn format(&mut self, ctx: &mut Context) {
		for value in self {
			value.format(ctx);
		}
	}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output> {
		match self.first_mut() {
			Some(value) => value.with_prefix_ws(ctx, visitor),
			None => None,
		}
	}
}

impl Format for ! {
	fn input_range(&mut self, _ctx: &mut Context) -> Option<AstRange> {
		*self
	}

	fn with_output(&mut self, _ctx: &mut Context, _f: &mut impl FnMut(&mut AstStr, &mut Context)) {
		*self
	}

	fn format(&mut self, _ctx: &mut Context) {
		*self
	}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, _ctx: &mut Context, _visitor: &mut V) -> Option<V::Output> {
		*self
	}
}

impl<T> Format for PhantomData<T> {
	fn input_range(&mut self, _ctx: &mut Context) -> Option<AstRange> {
		None
	}

	fn with_output(&mut self, _ctx: &mut Context, _f: &mut impl FnMut(&mut AstStr, &mut Context)) {}

	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, _ctx: &mut Context, _visitor: &mut V) -> Option<V::Output> {
		None
	}
}

impl Format for () {
	fn input_range(&mut self, _ctx: &mut Context) -> Option<AstRange> {
		None
	}

	fn with_output(&mut self, _ctx: &mut Context, _f: &mut impl FnMut(&mut AstStr, &mut Context)) {}

	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, _ctx: &mut Context, _visitor: &mut V) -> Option<V::Output> {
		None
	}
}

macro tuple_impl ($N:literal, $($T:ident),* $(,)?) {
	#[derive(Debug, Format)]
	#[expect(non_snake_case)]
	struct ${concat( Tuple, $N )}< $( $T, )* > {
		$( $T: $T, )*
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Format,)* > Format for ( $($T,)* ) {
		fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange> {
			let ( $($T,)* ) = self;

			let mut compute_range = ComputeRange::default();
			$( compute_range.add($T, ctx); )*
			compute_range.finish()
		}

		fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context)) {
			let ( $($T,)* ) = self;

			$(
				$T.with_output(ctx, f);
			)*
		}

		fn format(&mut self, ctx: &mut Context) {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format(ctx)
		}

		fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_prefix_ws(ctx, visitor)
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl Format for AstStr {
	fn input_range(&mut self, _ctx: &mut Context) -> Option<AstRange> {
		match self.repr() {
			AstStrRepr::AstRange(range) => Some(range),
			AstStrRepr::String(_) => None,
		}
	}

	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut Self, &mut Context)) {
		f(self, ctx);
	}

	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, _ctx: &mut Context, _visitor: &mut V) -> Option<V::Output> {
		None
	}
}

impl<T: ArenaData<Data: Format>> Format for ArenaIdx<T> {
	fn input_range(&mut self, ctx: &mut Context) -> Option<AstRange> {
		T::ARENA.get(self).input_range(ctx)
	}

	fn with_output(&mut self, ctx: &mut Context, f: &mut impl FnMut(&mut AstStr, &mut Context)) {
		T::ARENA.get(self).with_output(ctx, f);
	}

	fn format(&mut self, ctx: &mut Context) {
		T::ARENA.get(self).format(ctx);
	}

	fn with_prefix_ws<V: WhitespaceVisitor>(&mut self, ctx: &mut Context, visitor: &mut V) -> Option<V::Output> {
		T::ARENA.get(self).with_prefix_ws(ctx, visitor)
	}
}

/// Format context
pub struct Context<'a, 'input> {
	input:        &'input str,
	config:       &'a Config,
	indent_depth: usize,
	replacements: &'a mut Replacements,
}

impl<'a, 'input> Context<'a, 'input> {
	/// Creates a new context
	#[must_use]
	pub const fn new(input: &'input str, replacements: &'a mut Replacements, config: &'a Config) -> Self {
		Self {
			input,
			config,
			indent_depth: 0,
			replacements,
		}
	}

	/// Returns the input
	#[must_use]
	pub const fn input(&self) -> &'input str {
		self.input
	}

	/// Returns the string of a string
	#[must_use]
	pub fn str(&mut self, s: &AstStr) -> &'input str {
		s.str(self.input)
	}

	/// Returns the config
	#[must_use]
	pub const fn config(&self) -> &'a Config {
		self.config
	}

	/// Returns the indentation level
	#[must_use]
	pub const fn indent(&self) -> usize {
		self.indent_depth
	}

	/// Replaces a string
	pub fn replace(&mut self, s: &AstStr, replacement: impl Into<Replacement>) {
		self.replacements.add(self.config, s, s.str(self.input), replacement);
	}

	/// Runs `f` with a further indentation level
	pub fn with_indent<O>(&mut self, f: impl FnOnce(&mut Self) -> O) -> O {
		self.indent_depth += 1;
		let output = f(self);
		self.indent_depth -= 1;
		output
	}

	/// Runs `f` with one less indentation level
	pub fn without_indent<O>(&mut self, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		self.with_indent_offset(-1, f)
	}

	/// Runs `f` with one less indentation level if `pred` is true, otherwise
	/// runs it with the current indent
	pub fn without_indent_if<O>(&mut self, pred: bool, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		self.with_indent_offset_if(-1, pred, f)
	}

	/// Runs `f` with an indentation offset of `offset`
	pub fn with_indent_offset<O>(&mut self, offset: isize, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		let prev_depth = self.indent_depth;
		self.indent_depth = prev_depth.saturating_add_signed(offset);
		let output = f(self);
		self.indent_depth = prev_depth;
		output
	}

	/// Runs `f` with an indentation offset of `offset` if `pred` is true
	pub fn with_indent_offset_if<O>(
		&mut self,
		offset: isize,
		pred: bool,
		f: impl for<'b> FnOnce(&'b mut Self) -> O,
	) -> O {
		match pred {
			true => self.with_indent_offset(offset, f),
			false => f(self),
		}
	}

	#[doc(hidden)]
	pub const fn set_indent_depth(&mut self, indent_depth: usize) {
		self.indent_depth = indent_depth;
	}
}

/// A formatting function
pub trait FormatFn<T: ?Sized> = Fn(&mut T, &mut Context);

/// Formats an arena value
pub fn arena<T: ArenaData>(f: impl FormatFn<T::Data>) -> impl FormatFn<ArenaIdx<T>> {
	move |idx, ctx| {
		let mut value = T::ARENA.get(idx);
		f(&mut value, ctx);
	}
}

/// Formats an `Option<Self>` with `f` if it is `Some`.
pub fn format_option_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Option<T>> {
	move |value, ctx| {
		if let Some(value) = value {
			f(value, ctx);
		}
	}
}

/// Formats *all* items of a `Vec<T>` with `f`
pub fn format_vec_each_with_all<T>(f: impl FormatFn<T>) -> impl FormatFn<Vec<T>> {
	move |values, ctx| {
		for value in values {
			f(value, ctx);
		}
	}
}

/// Formats a `Vec<T>` (except the first element) with `f`
pub fn format_vec_each_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Vec<T>> {
	move |values, ctx| {
		if let Some(values) = values.get_mut(1..) {
			for value in values {
				f(value, ctx);
			}
		}
	}
}

/// Item range computer
#[derive(Clone, Copy, Default, Debug)]
pub struct ComputeRange {
	cur: Option<AstRange>,
}

impl ComputeRange {
	/// Adds an ast range to this
	pub const fn add_range(&mut self, range: AstRange) {
		match &mut self.cur {
			Some(cur) => cur.end = range.end,
			None => self.cur = Some(range),
		}
	}

	/// Adds the next item to this
	pub fn add<T: Format>(&mut self, mut item: T, ctx: &mut Context) {
		let Some(range) = item.input_range(ctx) else { return };
		self.add_range(range);
	}

	/// Adds several items to this
	pub fn extend<I>(&mut self, items: I, ctx: &mut Context)
	where
		I: IntoIterator<Item: Format>,
	{
		for item in items {
			self.add(item, ctx);
		}
	}

	/// Returns the computed range
	#[must_use]
	pub const fn finish(&mut self) -> Option<AstRange> {
		self.cur
	}
}
