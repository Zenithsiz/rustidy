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

// Modules
mod tag;
pub mod vec;
pub mod whitespace;

// Exports
pub use {
	self::{whitespace::{WhitespaceFormat, WhitespaceFormatKind}},
	rustidy_macros::{Format, Formattable},
	tag::{FormatTag, FormatTags},
};

// Imports
use {
	crate as rustidy_format,
	arcstr::ArcStr,
	core::{marker::PhantomData, ops::ControlFlow},
	rustidy_util::{ArenaData, ArenaIdx, AstStr, Config, Oob, Whitespace},
	std::borrow::Cow,
};

/// Formattable types
pub trait Formattable {
	/// Accesses the prefix whitespace of this type.
	///
	/// # Return
	/// - `Ok()` if the prefix whitespace was found.
	/// - `Err(Break(()))` if no prefix whitespace existed and the type wasn't empty
	/// - `Err(Continue(()))` if no prefix whitespace existed but the type was empty.
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>>;

	/// Returns if the prefix whitespace is pure.
	fn prefix_ws_is_pure(&mut self, ctx: &mut Context) -> Option<bool> {
		self
			.with_prefix_ws(ctx, &mut |ws, _ctx| ws.is_pure())
			.ok()
	}

	/// Joins a string as a prefix onto the prefix whitespace of this type.
	fn prefix_ws_join_prefix(&mut self, ctx: &mut Context, ws: Whitespace) -> Result<(), Whitespace> {
		let mut join_ws = Some(ws);
		let _ = self
			.with_prefix_ws(
				ctx,
				&mut |ws, _| {
					ws
						.join_prefix(
							join_ws
								.take()
								.expect("`with_prefix_ws` called multiple times")
						);
				}
			);

		match join_ws {
			Some(ws) => Err(ws),
			None => Ok(()),
		}
	}

	/// Iterates over all strings in this type.
	///
	/// # Returns
	/// - `Break()` if `f` returned `Break()`
	/// - `Continue(true)` if this type was empty.
	/// - `Continue(false)` if this type was non-empty.
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool>;

	/// Returns the formatting output for this type, without formatting it.
	fn format_output(&mut self, ctx: &mut Context) -> FormatOutput;
}

/// Formatting output
#[derive(Clone, Copy, Debug)]
#[must_use = "Should not ignore format output"]
pub struct FormatOutput {
	/// Prefix whitespace length, if any
	pub prefix_ws_len: Option<usize>,

	/// Total length of this type
	pub len:           usize,

	/// Number of newlines in the input
	pub newlines:      usize,

	/// Whether the type was empty
	pub is_empty:      bool,

	/// Whether the type was blank
	pub is_blank:      bool,
}

impl FormatOutput {
	/// Returns if this format output has any prefix whitespace
	#[must_use]
	pub const fn has_prefix_ws(&self) -> bool {
		self.prefix_ws_len.is_some()
	}

	/// Returns if this format output has any newlines
	#[must_use]
	pub const fn has_newlines(&self) -> bool {
		self.newlines != 0
	}

	/// Returns the length of this type, excluding the prefix whitespace, if any
	// TODO: Rename this to just `len` and `Self::len` to `total_len`?.
	#[must_use]
	pub fn len_without_prefix_ws(&self) -> usize {
		self.len - self.prefix_ws_len.unwrap_or(0)
	}

	/// Joins two format outputs.
	///
	/// You must ensure `lhs` was formatted *before* `rhs`,
	/// to ensure the semantics of the type.
	///
	/// It's fine if `lhs` and `rhs` have any "holes", so long
	/// as you ensure the above point.
	pub const fn join(lhs: Self, rhs: Self) -> Self {
		Self {
			prefix_ws_len: match lhs.prefix_ws_len {
				Some(prefix_ws_len) => Some(prefix_ws_len),
				None => match lhs.len == 0 {
					true => rhs.prefix_ws_len,
					false => None,
				},
			},
			len: lhs.len + rhs.len,
			newlines: lhs.newlines + rhs.newlines,
			is_empty: lhs.is_empty && rhs.is_empty,
			is_blank: lhs.is_blank && rhs.is_blank,
		}
	}

	/// Appends a format output to this one.
	///
	/// See [`join`](Self::join) for details.
	pub const fn append(&mut self, other: Self) {
		*self = Self::join(*self, other);
	}

	/// Appends this format output to `output`.
	///
	/// See [`join`](Self::join) for details.
	pub const fn append_to(self, output: &mut Self) {
		output.append(self);
	}
}

impl<const N: usize> From<[Self; N]> for FormatOutput {
	fn from(outputs: [Self; N]) -> Self {
		outputs.into_iter().collect()
	}
}

impl FromIterator<Self> for FormatOutput {
	fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
		iter
			.into_iter()
			.fold(Self::default(), Self::join)
	}
}

impl Default for FormatOutput {
	fn default() -> Self {
		Self {
			prefix_ws_len: None,
			len: 0,
			newlines: 0,
			is_empty: true,
			is_blank: true,
		}
	}
}

/// Type formatting
pub trait Format<PrefixWs, Args>: Formattable {
	/// Formats this type.
	// TODO: Rename this to be less confusing with `Context::format`?
	fn format(
		&mut self,
		ctx: &mut Context,
		prefix_ws: PrefixWs,
		args: Args
	) -> FormatOutput;
}

impl<T: Formattable> Formattable for &'_ mut T {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		(**self).with_prefix_ws(ctx, f)
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}

	fn format_output(&mut self, ctx: &mut Context) -> FormatOutput {
		(**self).format_output(ctx)
	}
}

impl<T: Format<PrefixWs, Args>, PrefixWs, Args> Format<PrefixWs, Args> for &'_ mut T {
	fn format(
		&mut self,
		ctx: &mut Context,
		prefix_ws: PrefixWs,
		args: Args
	) -> FormatOutput {
		(**self).format(ctx, prefix_ws, args)
	}
}

impl<T: Formattable> Formattable for Box<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		(**self).with_prefix_ws(ctx, f)
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}

	fn format_output(&mut self, ctx: &mut Context) -> FormatOutput {
		(**self).format_output(ctx)
	}
}

impl<T: Format<PrefixWs, Args>, PrefixWs, Args> Format<PrefixWs, Args> for Box<T> {
	fn format(
		&mut self,
		ctx: &mut Context,
		prefix_ws: PrefixWs,
		args: Args
	) -> FormatOutput {
		(**self).format(ctx, prefix_ws, args)
	}
}

impl<T: Formattable> Formattable for Option<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		match self {
			Self::Some(value) => value.with_prefix_ws(ctx, f),
			Self::None => Err(ControlFlow::Continue(())),
		}
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		match self {
			Some(value) => value.with_strings(ctx, exclude_prefix_ws, f),
			None => ControlFlow::Continue(true),
		}
	}

	fn format_output(&mut self, ctx: &mut Context) -> FormatOutput {
		match self {
			Self::Some(value) => value.format_output(ctx),
			Self::None => FormatOutput::default(),
		}
	}
}

impl<T: Format<PrefixWs, Args>, PrefixWs, Args> Format<PrefixWs, Args> for Option<T> {
	fn format(
		&mut self,
		ctx: &mut Context,
		prefix_ws: PrefixWs,
		args: Args
	) -> FormatOutput {
		match self {
			Some(value) => value.format(ctx, prefix_ws, args),
			_ => FormatOutput::default(),
		}
	}
}

impl Formattable for ! {
	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		*self
	}

	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		*self
	}

	fn format_output(&mut self, _ctx: &mut Context) -> FormatOutput {
		*self
	}
}

impl<PrefixWs, Args> Format<PrefixWs, Args> for ! {
	fn format(
		&mut self,
		_ctx: &mut Context,
		_prefix_ws: PrefixWs,
		_args: Args
	) -> FormatOutput {
		*self
	}
}

impl<T> Formattable for PhantomData<T> {
	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		Err(ControlFlow::Continue(()))
	}

	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		ControlFlow::Continue(true)
	}

	fn format_output(&mut self, _ctx: &mut Context) -> FormatOutput {
		FormatOutput::default()
	}
}

impl<T> Format<(), ()> for PhantomData<T> {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: (), _args: ()) -> FormatOutput {
		FormatOutput::default()
	}
}

impl Formattable for () {
	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		Err(ControlFlow::Continue(()))
	}

	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		ControlFlow::Continue(true)
	}

	fn format_output(&mut self, _ctx: &mut Context) -> FormatOutput {
		FormatOutput::default()
	}
}

impl Format<(), ()> for () {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: (), _args: ()) -> FormatOutput {
		FormatOutput::default()
	}
}

macro tuple_impl(
	$N:literal, $($T:ident),* $(,)?
) {
	#[derive(Debug, Formattable, Format)]
	#[expect(non_snake_case)]
	struct ${concat( Tuple, $N )}< $( $T, )* > {
		$( $T: $T, )*
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Formattable,)* > Formattable for ( $($T,)* ) {
		fn with_prefix_ws<O>(
			&mut self,
			ctx: &mut Context,
			f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
		) -> Result<O, ControlFlow<()>> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_prefix_ws(ctx, f)
		}

		fn with_strings<O>(
			&mut self,
			ctx: &mut Context,
			exclude_prefix_ws: bool,
			f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
		) -> ControlFlow<O, bool> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_strings(ctx, exclude_prefix_ws, f)
		}

		fn format_output(&mut self, ctx: &mut Context) -> FormatOutput {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format_output(ctx)
		}
	}

	// TODO: Make this impl generic for all prefix whitespace/args?
	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Format<WhitespaceConfig, ()>,)*> Format<WhitespaceConfig, ()> for ( $($T,)* ) {
		fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: ()) -> FormatOutput {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format(ctx, prefix_ws, args)
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl Formattable for AstStr {
	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		match self.is_empty() {
			true => Err(ControlFlow::Continue(())),
			false => Err(ControlFlow::Break(())),
		}
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		_exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut Self,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		f(self, ctx)?;

		ControlFlow::Continue(self.is_empty())
	}

	fn format_output(&mut self, _ctx: &mut Context) -> FormatOutput {
		// TODO: Optimize these by not iterating over the string multiple times.
		FormatOutput {
			prefix_ws_len: None,
			len: self.len(),
			newlines: self.count_newlines(),
			is_empty: self.is_empty(),
			is_blank: self.is_blank(),
		}
	}
}

impl<T: ArenaData + Formattable> Formattable for ArenaIdx<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace,&mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		(**self).with_prefix_ws(ctx, f)
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr,&mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}

	fn format_output(&mut self, ctx: &mut Context) -> FormatOutput {
		(**self).format_output(ctx)
	}
}

impl<T: ArenaData + Format<PrefixWs, Args>, PrefixWs, Args> Format<PrefixWs, Args> for ArenaIdx<T> {
	fn format(
		&mut self,
		ctx: &mut Context,
		prefix_ws: PrefixWs,
		args: Args
	) -> FormatOutput {
		(**self).format(ctx, prefix_ws, args)
	}
}

/// Format context
pub struct Context<'a> {
	input:        ArcStr,
	config:       Cow<'a, Config>,
	indent_depth: usize,
	tags:         Oob<'a, FormatTags>,
}

impl<'a> Context<'a> {
	/// Creates a new context
	#[must_use]
	pub fn new(input: impl Into<ArcStr>, config: &'a Config) -> Self {
		Self {
			input: input.into(),
			config: Cow::Borrowed(config),
			indent_depth: 0,
			tags: Oob::Owned(FormatTags::new()),
		}
	}

	/// Formats a value
	pub fn format<T, PrefixWs>(&mut self, value: &mut T, prefix_ws: PrefixWs) -> FormatOutput
	where
		T: Format<PrefixWs, ()>
	{
		self.format_with(value, prefix_ws, ())
	}

	/// Formats a value with arguments
	pub fn format_with<T, PrefixWs, A>(&mut self, value: &mut T, prefix_ws: PrefixWs, args: A) -> FormatOutput
	where
		T: Format<PrefixWs, A>
	{
		match self.config().skip {
			true => value.format_output(self),
			false => value.format(self, prefix_ws, args),
		}
	}

	/// Returns the input
	#[must_use]
	pub const fn input(&self) -> &ArcStr {
		&self.input
	}

	/// Returns the config
	#[must_use]
	pub fn config(&self) -> &Config {
		&self.config
	}

	/// Returns the config mutably
	#[must_use]
	pub fn config_mut(&mut self) -> &mut Config {
		Cow::to_mut(&mut self.config)
	}

	/// Returns the indentation level
	#[must_use]
	pub const fn indent(&self) -> usize {
		self.indent_depth
	}

	/// Runs `f` with a further indentation level
	pub fn with_indent<O>(&mut self, f: impl FnOnce(&mut Self) -> O) -> O {
		self.with_indent_offset(1, f)
	}

	/// Runs `f` with a further indentation level if `pred` is true
	pub fn with_indent_if<O>(&mut self, pred: bool, f: impl FnOnce(&mut Self) -> O) -> O {
		self.with_indent_offset_if(1, pred, f)
	}

	// TODO: Should `without_indent[_if]` be removed?
	/// Runs `f` with one less indentation level
	pub fn without_indent<O>(&mut self, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		self.with_indent_offset(-1, f)
	}

	/// Runs `f` with one less indentation level if `pred` is true, otherwise
	/// runs it with the current indent
	pub fn without_indent_if<O>(
		&mut self,
		pred: bool,
		f: impl for<'b> FnOnce(&'b mut Self) -> O
	) -> O {
		self.with_indent_offset_if(-1, pred, f)
	}

	/// Runs `f` with an indentation offset of `offset`
	pub fn with_indent_offset<O>(
		&mut self,
		offset: i16,
		f: impl for<'b> FnOnce(&'b mut Self) -> O
	) -> O {
		let prev_depth = self.indent_depth;
		self.indent_depth = prev_depth
			.saturating_add_signed(isize::from(offset));
		let output = f(self);
		self.indent_depth = prev_depth;
		output
	}

	/// Runs `f` with an indentation offset of `offset` if `pred` is true
	pub fn with_indent_offset_if<O>(
		&mut self,
		offset: i16,
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

	/// Creates a sub-context.
	///
	/// Sub contexts have their own configuration
	pub fn sub_context(&mut self) -> Context<'_> {
		Context {
			input: ArcStr::clone(&self.input),
			config: Cow::Borrowed(&self.config),
			indent_depth: self.indent_depth,
			tags: Oob::Borrowed(&mut self.tags),
		}
	}

	/// Adds a tag.
	///
	/// Returns if the tag was present
	pub fn add_tag(&mut self, tag: FormatTag) -> bool {
		self.tags.add(tag)
	}

	/// Removes a tag.
	///
	/// Returns if the tag was present
	pub fn remove_tag(&mut self, tag: FormatTag) -> bool {
		self.tags.remove(tag)
	}

	/// Sets whether a tag is present.
	///
	/// Returns if the tag was present
	pub fn set_tag(&mut self, tag: FormatTag, present: bool) -> bool {
		self.tags.set(tag, present)
	}

	/// Returns if a tag exists
	#[must_use]
	pub fn has_tag(&self, tag: FormatTag) -> bool {
		self.tags.contains(tag)
	}

	/// Runs `f` with a tag, removing it after
	pub fn with_tag<O>(&mut self, tag: FormatTag, f: impl FnOnce(&mut Self) -> O) -> O {
		let was_present = self.add_tag(tag);
		let output = f(self);
		self.set_tag(tag, was_present);

		output
	}

	/// Runs `f` with a tag if `pred` is true, removing it after
	pub fn with_tag_if<O>(
		&mut self,
		pred: bool,
		tag: FormatTag,
		f: impl FnOnce(&mut Self) -> O
	) -> O {
		match pred {
			true => self.with_tag(tag, f),
			false => f(self),
		}
	}

	/// Runs `f` without a tag, adding it after if it existed
	pub fn without_tag<O>(&mut self, tag: FormatTag, f: impl FnOnce(&mut Self) -> O) -> O {
		let was_present = self.remove_tag(tag);
		let output = f(self);
		self.set_tag(tag, was_present);

		output
	}

	/// Runs `f` without a tag if `pred` is true, adding it after if it existed
	pub fn without_tag_if<O>(
		&mut self,
		pred: bool,
		tag: FormatTag,
		f: impl FnOnce(&mut Self) -> O
	) -> O {
		match pred {
			true => self.without_tag(tag, f),
			false => f(self),
		}
	}
}

/// Whitespace formatting configuration
#[derive(Clone, Copy, Debug)]
pub struct WhitespaceConfig {
	format: Option<WhitespaceFormatKind>,
}

const _: () = const {
	assert!(size_of::<WhitespaceConfig>() <= 8);
};
