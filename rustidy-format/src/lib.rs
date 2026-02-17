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
pub mod str;
mod tag;
pub mod vec;
pub mod whitespace;

// Exports
pub use {
	self::{
		str::AstStrFormat,
		whitespace::{WhitespaceFormat, WhitespaceFormatKind},
	},
	rustidy_macros::{Format, Formattable},
	tag::FormatTag,
};

// Imports
use {
	crate as rustidy_format,
	core::{marker::PhantomData, mem, ops::ControlFlow},
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
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>>;

	/// Returns if the prefix whitespace is pure.
	fn prefix_ws_is_pure(&mut self, ctx: &mut Context) -> Option<bool> {
		self.with_prefix_ws(ctx, &mut |ws, ctx| ws.is_pure(ctx)).ok()
	}

	/// Joins a string as a prefix onto the prefix whitespace of this type.
	fn prefix_ws_join_prefix(&mut self, ctx: &mut Context, ws: Whitespace) -> Result<(), Whitespace> {
		let mut join_ws = Some(ws);
		let _ = self.with_prefix_ws(ctx, &mut |ws, _| {
			ws.join_prefix(join_ws.take().expect("`with_prefix_ws` called multiple times"));
		});

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
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool>;

	/// Returns if this type is blank
	fn is_blank(&mut self, ctx: &mut Context, exclude_prefix_ws: bool) -> bool {
		fn is_blank(s: &mut AstStr, ctx: &mut Context<'_, '_>) -> ControlFlow<()> {
			match AstStr::is_blank(s, ctx.input) {
				true => ControlFlow::Continue(()),
				false => ControlFlow::Break(()),
			}
		}

		self.with_strings(ctx, exclude_prefix_ws, &mut is_blank).is_continue()
	}
}

/// Formatting output
#[derive(Clone, Copy, Debug)]
#[must_use = "Should not ignore format output"]
pub struct FormatOutput {
	/// Prefix whitespace length, if any
	pub prefix_ws_len: Option<usize>,

	/// Total length of this type
	pub len: usize,

	/// Whether the type was empty
	pub is_empty: bool,

	/// Whether the type has newlines
	pub has_newlines: bool,
}

impl FormatOutput {
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
			len:           lhs.len + rhs.len,
			is_empty:      lhs.is_empty && rhs.is_empty,
			has_newlines:  lhs.has_newlines || rhs.has_newlines,
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

impl Default for FormatOutput {
	fn default() -> Self {
		Self {
			prefix_ws_len: None,
			len:           0,
			is_empty:      true,
			has_newlines:  false,
		}
	}
}

/// Type formatting
pub trait Format<Args>: Formattable {
	/// Formats this type, using `prefix_ws` to format it's prefix whitespace, if any.
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut Args) -> FormatOutput;
}

impl<T: Formattable> Formattable for &'_ mut T {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		(**self).with_prefix_ws(ctx, f)
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}
}

impl<T: Format<Args>, Args> Format<Args> for &'_ mut T {
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut Args) -> FormatOutput {
		(**self).format(ctx, prefix_ws, args)
	}
}

impl<T: Formattable> Formattable for Box<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		(**self).with_prefix_ws(ctx, f)
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}
}

impl<T: Format<Args>, Args> Format<Args> for Box<T> {
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut Args) -> FormatOutput {
		(**self).format(ctx, prefix_ws, args)
	}
}

impl<T: Formattable> Formattable for Option<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
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
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		match self {
			Some(value) => value.with_strings(ctx, exclude_prefix_ws, f),
			None => ControlFlow::Continue(true),
		}
	}
}

impl<T: Format<Args>, Args> Format<Args> for Option<T> {
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut Args) -> FormatOutput {
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
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		*self
	}

	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		*self
	}
}

impl Format<()> for ! {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: WhitespaceConfig, (): &mut ()) -> FormatOutput {
		*self
	}
}

impl<T> Formattable for PhantomData<T> {
	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		Err(ControlFlow::Continue(()))
	}

	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		ControlFlow::Continue(true)
	}
}

impl<T> Format<()> for PhantomData<T> {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: WhitespaceConfig, (): &mut ()) -> FormatOutput {
		FormatOutput::default()
	}
}

impl Formattable for () {
	fn with_prefix_ws<O>(
		&mut self,
		_ctx: &mut Context,
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		Err(ControlFlow::Continue(()))
	}

	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		ControlFlow::Continue(true)
	}
}

impl Format<()> for () {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: WhitespaceConfig, (): &mut ()) -> FormatOutput {
		FormatOutput::default()
	}
}

macro tuple_impl ($N:literal, $($T:ident),* $(,)?) {
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
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Format<()>,)*> Format<()> for ( $($T,)* ) {
		fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut ()) -> FormatOutput {
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
		_f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
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
		f: &mut impl FnMut(&mut Self, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		f(self, ctx)?;

		ControlFlow::Continue(self.is_empty())
	}
}

impl Format<()> for AstStr {
	fn format(&mut self, ctx: &mut Context, _prefix_ws: WhitespaceConfig, (): &mut ()) -> FormatOutput {
		self.format_output(ctx)
	}
}

impl<T: ArenaData<Data: Formattable>> Formattable for ArenaIdx<T> {
	fn with_prefix_ws<O>(
		&mut self,
		ctx: &mut Context,
		f: &mut impl FnMut(&mut Whitespace, &mut Context) -> O,
	) -> Result<O, ControlFlow<()>> {
		self.get_mut().with_prefix_ws(ctx, f)
	}

	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O, bool> {
		self.get_mut().with_strings(ctx, exclude_prefix_ws, f)
	}
}

impl<T: ArenaData<Data: Format<Args>>, Args> Format<Args> for ArenaIdx<T> {
	fn format(&mut self, ctx: &mut Context, prefix_ws: WhitespaceConfig, args: &mut Args) -> FormatOutput {
		self.get_mut().format(ctx, prefix_ws, args)
	}
}

/// Format context
pub struct Context<'a, 'input> {
	input:        &'input str,
	config:       Cow<'a, Config>,
	indent_depth: usize,
	tags:         Oob<'a, Vec<FormatTag>>,
}

impl<'a, 'input> Context<'a, 'input> {
	/// Creates a new context
	#[must_use]
	pub const fn new(input: &'input str, config: &'a Config) -> Self {
		Self {
			input,
			config: Cow::Borrowed(config),
			indent_depth: 0,
			tags: Oob::Owned(vec![]),
		}
	}

	/// Returns the input
	#[must_use]
	pub const fn input(&self) -> &'input str {
		self.input
	}

	/// Returns the string of a string
	#[must_use]
	pub fn str(&mut self, s: &AstStr) -> Cow<'input, str> {
		s.str(self.input)
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
	pub fn without_indent_if<O>(&mut self, pred: bool, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		self.with_indent_offset_if(-1, pred, f)
	}

	/// Runs `f` with an indentation offset of `offset`
	pub fn with_indent_offset<O>(&mut self, offset: i16, f: impl for<'b> FnOnce(&'b mut Self) -> O) -> O {
		let prev_depth = self.indent_depth;
		self.indent_depth = prev_depth.saturating_add_signed(isize::from(offset));
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
	pub fn sub_context(&mut self) -> Context<'_, 'input> {
		Context {
			input:        self.input,
			config:       Cow::Borrowed(&self.config),
			indent_depth: self.indent_depth,
			tags:         Oob::Borrowed(&mut self.tags),
		}
	}

	/// Returns all tags
	pub fn tags(&self) -> impl Iterator<Item = FormatTag> {
		self.tags.iter().copied()
	}

	/// Returns if this context has a tag
	#[must_use]
	pub fn has_tag(&self, tag: impl Into<FormatTag>) -> bool {
		let tag = tag.into();
		self.tags().any(|cur_tag| cur_tag == tag)
	}

	/// Returns if this context has a tag and removes it
	#[must_use]
	pub fn take_tag(&mut self, tag: impl Into<FormatTag>) -> bool {
		let tag = tag.into();

		let prev_len = self.tags.len();
		self.tags.retain(|cur_tag| *cur_tag != tag);
		self.tags.len() != prev_len
	}

	/// Calls `f` with tags `tags` added to this context
	pub fn with_tags<O>(&mut self, tags: impl IntoIterator<Item = FormatTag>, f: impl FnOnce(&mut Self) -> O) -> O {
		let tags_len = self.tags.len();

		for tag in tags {
			self.tags.push(tag);
		}
		let output = f(self);
		if self.tags.len() != tags_len {
			self.tags.truncate(tags_len);
		}

		output
	}

	/// Calls `f` with tag `tag` added to this context
	pub fn with_tag<O>(&mut self, tag: impl Into<FormatTag>, f: impl FnOnce(&mut Self) -> O) -> O {
		self.with_tags([tag.into()], f)
	}

	/// Calls `f` with tag `tag` added to this context if `pred` is true
	pub fn with_tag_if<O>(&mut self, pred: bool, tag: impl Into<FormatTag>, f: impl FnOnce(&mut Self) -> O) -> O {
		match pred {
			true => self.with_tag(tag, f),
			false => f(self),
		}
	}

	/// Calls `f` with all tags removed.
	pub fn without_tags<O>(&mut self, f: impl FnOnce(&mut Self) -> O) -> O {
		// TODO: Just add an offset to the start of the new tags
		//       to reduce an allocation?
		let tags = mem::take(&mut *self.tags);
		let output = f(self);
		*self.tags = tags;

		output
	}
}

/// Whitespace formatting configuration
#[derive(Clone, Copy)]
pub struct WhitespaceConfig {
	format: Option<WhitespaceFormatKind>,
}

const _: () = const { assert!(size_of::<WhitespaceConfig>() <= 8) };
