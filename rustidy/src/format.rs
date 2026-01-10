//! Formattable types

// Modules
pub mod config;

// Exports
pub use {self::config::Config, rustidy_macros::Format};

// Imports
use {
	crate::{
		Arenas,
		ParserStr,
		Replacement,
		Replacements,
		arena::{ArenaData, ArenaIdx, WithArena},
		ast::whitespace::Whitespace,
		parser::ParserRange,
	},
	core::marker::PhantomData,
};

/// Formattable read-only utils
// TODO: Better name?
pub trait FormatRef {
	/// Returns the range of this type
	fn range(&self, ctx: &Context) -> Option<ParserRange>;

	/// Returns the length of this type
	fn len(&self, ctx: &Context) -> usize;

	/// Returns if this type is empty
	fn is_empty(&self, ctx: &Context) -> bool {
		self.len(ctx) == 0
	}
}

/// Formattable type
pub trait Format: FormatRef {
	/// Formats this type
	fn format(&mut self, ctx: &mut Context);

	/// Uses the first whitespace of this type, if any.
	///
	/// Returns if successfully used.
	fn with_prefix_ws<R, F>(&mut self, ctx: &mut Context, f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy;

	/// Remove the prefix whitespace, if any
	fn prefix_ws_remove(&mut self, ctx: &mut Context) {
		self.with_prefix_ws(ctx, |whitespace, ctx| whitespace.remove(ctx));
	}

	/// Sets the prefix whitespace to a single space
	fn prefix_ws_set_single(&mut self, ctx: &mut Context) {
		self.with_prefix_ws(ctx, |whitespace, ctx| whitespace.set_single(ctx));
	}

	/// Sets the prefix whitespace to the current indentation
	fn prefix_ws_set_indent(&mut self, ctx: &mut Context, offset: isize, remove_if_empty: bool) {
		self.with_prefix_ws(ctx, |whitespace, ctx| {
			whitespace.set_indent(ctx, offset, remove_if_empty);
		});
	}
}

impl<T: FormatRef> FormatRef for &'_ T {
	fn range(&self, ctx: &Context) -> Option<ParserRange> {
		(**self).range(ctx)
	}

	fn len(&self, ctx: &Context) -> usize {
		(**self).len(ctx)
	}
}

impl<T: FormatRef> FormatRef for &'_ mut T {
	fn range(&self, ctx: &Context) -> Option<ParserRange> {
		(**self).range(ctx)
	}

	fn len(&self, ctx: &Context) -> usize {
		(**self).len(ctx)
	}
}

impl<T: Format> Format for &'_ mut T {
	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn with_prefix_ws<R, F>(&mut self, ctx: &mut Context, f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
		(**self).with_prefix_ws(ctx, f)
	}
}

impl<T: FormatRef> FormatRef for Box<T> {
	fn range(&self, ctx: &Context) -> Option<ParserRange> {
		(**self).range(ctx)
	}

	fn len(&self, ctx: &Context) -> usize {
		(**self).len(ctx)
	}
}

impl<T: Format> Format for Box<T> {
	fn format(&mut self, ctx: &mut Context) {
		(**self).format(ctx);
	}

	fn with_prefix_ws<R, F>(&mut self, ctx: &mut Context, f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
		(**self).with_prefix_ws(ctx, f)
	}
}

impl<T: FormatRef> FormatRef for Option<T> {
	fn range(&self, _ctx: &Context) -> Option<ParserRange> {
		None
	}

	fn len(&self, ctx: &Context) -> usize {
		match self {
			Some(value) => value.len(ctx),
			None => 0,
		}
	}
}

impl<T: Format> Format for Option<T> {
	fn format(&mut self, ctx: &mut Context) {
		if let Some(value) = self {
			value.format(ctx);
		}
	}

	fn with_prefix_ws<R, F>(&mut self, ctx: &mut Context, f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
		match self {
			Some(value) => value.with_prefix_ws(ctx, f),
			_ => None,
		}
	}
}

impl<T: FormatRef> FormatRef for Vec<T> {
	fn range(&self, ctx: &Context) -> Option<ParserRange> {
		let mut compute_range = ComputeRange::default();
		compute_range.extend(self, ctx);
		compute_range.finish()
	}

	fn len(&self, ctx: &Context) -> usize {
		self.iter().map(|value| value.len(ctx)).sum()
	}
}

impl<T: Format> Format for Vec<T> {
	fn format(&mut self, ctx: &mut Context) {
		for value in self {
			value.format(ctx);
		}
	}

	fn with_prefix_ws<R, F>(&mut self, ctx: &mut Context, f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
		match self.first_mut() {
			Some(value) => value.with_prefix_ws(ctx, f),
			None => None,
		}
	}
}

impl FormatRef for ! {
	fn range(&self, _ctx: &Context) -> Option<ParserRange> {
		*self
	}

	fn len(&self, _ctx: &Context) -> usize {
		*self
	}
}

impl Format for ! {
	fn format(&mut self, _ctx: &mut Context) {
		*self
	}

	fn with_prefix_ws<R, F>(&mut self, _ctx: &mut Context, _f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
		*self
	}
}

impl<T> FormatRef for PhantomData<T> {
	fn range(&self, _ctx: &Context) -> Option<ParserRange> {
		None
	}

	fn len(&self, _ctx: &Context) -> usize {
		0
	}
}

impl<T> Format for PhantomData<T> {
	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<R, F>(&mut self, _ctx: &mut Context, _f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
		None
	}
}

impl FormatRef for () {
	fn range(&self, _ctx: &Context) -> Option<ParserRange> {
		None
	}

	fn len(&self, _ctx: &Context) -> usize {
		0
	}
}

impl Format for () {
	fn format(&mut self, _ctx: &mut Context) {}

	fn with_prefix_ws<R, F>(&mut self, _ctx: &mut Context, _f: F) -> Option<R>
	where
		F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
	{
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
	impl< $($T: FormatRef,)* > FormatRef for ( $($T,)* ) {
		fn range(&self, ctx: &Context) -> Option<ParserRange> {
			let ( $($T,)* ) = self;

			let mut compute_range = ComputeRange::default();
			$( compute_range.add($T, ctx); )*
			compute_range.finish()
		}

		fn len(&self, ctx: &Context) -> usize {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.len(ctx)
		}
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Format,)* > Format for ( $($T,)* ) {
		fn format(&mut self, ctx: &mut Context) {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format(ctx)
		}

		fn with_prefix_ws<R, F>(&mut self, ctx: &mut Context, f: F) -> Option<R>
		where
			F: Fn(&mut Whitespace, &mut Context) -> R + Copy,
		{
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_prefix_ws(ctx, f)
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl<T: ArenaData<Data: FormatRef> + WithArena> FormatRef for ArenaIdx<T> {
	fn range(&self, ctx: &Context) -> Option<ParserRange> {
		ctx.arenas().get(*self).range(ctx)
	}

	fn len(&self, ctx: &Context) -> usize {
		ctx.arenas().get(*self).len(ctx)
	}
}

impl<T: ArenaData<Data: Format> + WithArena> Format for ArenaIdx<T> {
	fn format(&mut self, ctx: &mut Context) {
		ctx.arenas().get(*self).format(ctx);
	}

	fn with_prefix_ws<R, F: Fn(&mut Whitespace, &mut Context) -> R + Copy>(
		&mut self,
		ctx: &mut Context,
		f: F,
	) -> Option<R> {
		ctx.arenas().get(*self).with_prefix_ws(ctx, f)
	}
}

/// Format context
pub struct Context<'a, 'input> {
	input:        &'input str,
	config:       &'a Config,
	indent_depth: usize,
	replacements: &'a mut Replacements,
	arenas:       &'a Arenas,
}

impl<'a, 'input> Context<'a, 'input> {
	/// Creates a new context
	#[must_use]
	pub const fn new(
		input: &'input str,
		replacements: &'a mut Replacements,
		arenas: &'a Arenas,
		config: &'a Config,
	) -> Self {
		Self {
			input,
			config,
			indent_depth: 0,
			replacements,
			arenas,
		}
	}

	/// Returns the input
	#[must_use]
	pub const fn input(&self) -> &'input str {
		self.input
	}

	/// Returns the string of a string
	#[must_use]
	pub fn str(&mut self, s: ParserStr) -> &'input str {
		s.range(self.arenas).str(self.input)
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

	/// Returns the arenas
	#[must_use]
	pub const fn arenas(&self) -> &'a Arenas {
		self.arenas
	}

	/// Replaces a string
	pub fn replace(&mut self, s: ParserStr, replacement: impl Into<Replacement>) {
		self.replacements
			.add(s, s.range(self.arenas).str(self.input), replacement);
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

	#[cfg(test)]
	pub const fn set_indent_depth(&mut self, indent_depth: usize) {
		self.indent_depth = indent_depth;
	}
}

/// A formatting function
pub trait FormatFn<T: ?Sized> = Fn(&mut T, &mut Context);

/// Sets the prefix whitespace to the current indentation
pub fn prefix_ws_set_indent<T: Format>(offset: isize, remove_if_empty: bool) -> impl Fn(&mut T, &mut Context) {
	move |this, ctx| this.prefix_ws_set_indent(ctx, offset, remove_if_empty)
}

/// Formats an `Option<Self>` with `f` if it is `Some`.
pub fn format_option_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Option<T>> {
	move |value, ctx| {
		if let Some(value) = value {
			f(value, ctx);
		}
	}
}

/// Formats a `Vec<Self>` with `f`
pub fn format_vec_each_with<T>(f: impl FormatFn<T>) -> impl FormatFn<Vec<T>> {
	move |values, ctx| {
		for value in values {
			f(value, ctx);
		}
	}
}

/// Item range computer
#[derive(Clone, Copy, Default, Debug)]
pub struct ComputeRange {
	cur: Option<ParserRange>,
}

impl ComputeRange {
	/// Adds a parser range to this
	pub const fn add_range(&mut self, range: ParserRange) {
		match &mut self.cur {
			Some(cur) => cur.end = range.end,
			None => self.cur = Some(range),
		}
	}

	/// Adds the next item to this
	pub fn add<T: FormatRef>(&mut self, item: T, ctx: &Context) {
		let Some(range) = item.range(ctx) else { return };
		self.add_range(range);
	}

	/// Adds several items to this
	pub fn extend<I>(&mut self, items: I, ctx: &Context)
	where
		I: IntoIterator<Item: FormatRef>,
	{
		for item in items {
			self.add(item, ctx);
		}
	}

	/// Returns the computed range
	#[must_use]
	pub const fn finish(&mut self) -> Option<ParserRange> {
		self.cur
	}
}
