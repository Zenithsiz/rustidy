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
#[doc(hidden)]
pub mod whitespace;

// Exports
pub use {self::whitespace::WhitespaceFormat, rustidy_macros::Format, tag::FormatTag};

// Imports
use {
	crate as rustidy_format,
	core::{marker::PhantomData, mem, ops::ControlFlow},
	rustidy_util::{ArenaData, ArenaIdx, AstStr, Config, Oob, Whitespace},
	std::borrow::Cow,
};

/// Formattable types
pub trait Formattable {
	/// Iterates over all strings in this type
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O>;

	/// Returns the length of this type
	fn len(&mut self, ctx: &mut Context, exclude_prefix_ws: bool) -> usize {
		let mut len = 0;
		self.with_strings::<!>(ctx, exclude_prefix_ws, &mut |s, _ctx| {
			len += AstStr::len(s);
			ControlFlow::Continue(())
		});
		len
	}

	/// Returns if this type is empty
	fn is_empty(&mut self, ctx: &mut Context, exclude_prefix_ws: bool) -> bool {
		self.with_strings(ctx, exclude_prefix_ws, &mut |s, _ctx| match AstStr::is_empty(s) {
			true => ControlFlow::Continue(()),
			false => ControlFlow::Break(()),
		})
		.is_continue()
	}

	/// Returns if this type is blank
	fn is_blank(&mut self, ctx: &mut Context, exclude_prefix_ws: bool) -> bool {
		self.with_strings(
			ctx,
			exclude_prefix_ws,
			&mut |s, ctx| match AstStr::is_blank(s, ctx.input) {
				true => ControlFlow::Continue(()),
				false => ControlFlow::Break(()),
			},
		)
		.is_continue()
	}

	/// Returns if this type has newlines
	fn has_newlines(&mut self, ctx: &mut Context, exclude_prefix_ws: bool) -> bool {
		self.with_strings(
			ctx,
			exclude_prefix_ws,
			&mut |s, ctx| match AstStr::has_newlines(s, ctx.input) {
				true => ControlFlow::Break(()),
				false => ControlFlow::Continue(()),
			},
		)
		.is_break()
	}
}

/// Type formatting
pub trait Format<Args>: Formattable {
	/// Formats this type, using `prefix_ws` to format it's prefix whitespace, if any.
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut Args);
}

impl<T: Formattable> Formattable for &'_ mut T {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}
}

impl<T: Format<Args>, Args> Format<Args> for &'_ mut T {
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut Args) {
		(**self).format(ctx, prefix_ws, args);
	}
}

impl<T: Formattable> Formattable for Box<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		(**self).with_strings(ctx, exclude_prefix_ws, f)
	}
}

impl<T: Format<Args>, Args> Format<Args> for Box<T> {
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut Args) {
		(**self).format(ctx, prefix_ws, args);
	}
}

impl<T: Formattable> Formattable for Option<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		match self {
			Some(value) => value.with_strings(ctx, exclude_prefix_ws, f),
			None => ControlFlow::Continue(()),
		}
	}
}

impl<T: Format<Args>, Args> Format<Args> for Option<T> {
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut Args) {
		if let Some(value) = self {
			value.format(ctx, prefix_ws, args);
		}
	}
}

impl<T: Formattable> Formattable for Vec<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		for value in self {
			value.with_strings(ctx, exclude_prefix_ws, f)?;
		}

		ControlFlow::Continue(())
	}
}

impl<T, W, A> Format<VecArgs<W, A>> for Vec<T>
where
	T: Format<A>,
	W: WsFmtFn,
{
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut VecArgs<W, A>) {
		let [first, rest @ ..] = &mut **self else {
			return;
		};

		first.format(ctx, prefix_ws, &mut args.args);
		for value in rest {
			value.format(ctx, &args.rest_prefix_ws, &mut args.args);
		}
	}
}

/// Arguments for formatting a [`Vec<T>`]
pub struct VecArgs<W, A> {
	rest_prefix_ws: W,
	args:           A,
}

impl<W, A> VecArgs<W, A> {
	pub const fn new(rest_prefix_ws: W, args: A) -> Self {
		Self { rest_prefix_ws, args }
	}
}

impl<W> VecArgs<W, ()> {
	pub const fn from_prefix_ws(rest_prefix_ws: W) -> Self {
		Self {
			rest_prefix_ws,
			args: (),
		}
	}
}

impl Formattable for ! {
	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		*self
	}
}

impl Format<()> for ! {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: &impl WsFmtFn, (): &mut ()) {
		*self
	}
}

impl<T> Formattable for PhantomData<T> {
	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		ControlFlow::Continue(())
	}
}

impl<T> Format<()> for PhantomData<T> {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: &impl WsFmtFn, (): &mut ()) {}
}

impl Formattable for () {
	fn with_strings<O>(
		&mut self,
		_ctx: &mut Context,
		_exclude_prefix_ws: bool,
		_f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		ControlFlow::Continue(())
	}
}

impl Format<()> for () {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: &impl WsFmtFn, (): &mut ()) {}
}

macro tuple_impl ($N:literal, $($T:ident),* $(,)?) {
	#[derive(Debug, Format)]
	#[expect(non_snake_case)]
	struct ${concat( Tuple, $N )}< $( $T, )* > {
		$( $T: $T, )*
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Formattable,)* > Formattable for ( $($T,)* ) {
		fn with_strings<O>(
			&mut self,
			ctx: &mut Context,
			exclude_prefix_ws: bool,
			f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
		) -> ControlFlow<O> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.with_strings(ctx, exclude_prefix_ws, f)
		}
	}

	#[automatically_derived]
	#[expect(non_snake_case)]
	impl< $($T: Format<()>,)*> Format<()> for ( $($T,)* ) {
		fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut ()) {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.format(ctx, prefix_ws, args)
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

impl Formattable for AstStr {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		_exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut Self, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		f(self, ctx)
	}
}

impl Format<()> for AstStr {
	fn format(&mut self, _ctx: &mut Context, _prefix_ws: &impl WsFmtFn, (): &mut ()) {}
}

impl<T: ArenaData<Data: Formattable>> Formattable for ArenaIdx<T> {
	fn with_strings<O>(
		&mut self,
		ctx: &mut Context,
		exclude_prefix_ws: bool,
		f: &mut impl FnMut(&mut AstStr, &mut Context) -> ControlFlow<O>,
	) -> ControlFlow<O> {
		self.get_mut().with_strings(ctx, exclude_prefix_ws, f)
	}
}

impl<T: ArenaData<Data: Format<Args>>, Args> Format<Args> for ArenaIdx<T> {
	fn format(&mut self, ctx: &mut Context, prefix_ws: &impl WsFmtFn, args: &mut Args) {
		self.get_mut().format(ctx, prefix_ws, args);
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

/// A whitespace formatting function
pub trait WsFmtFn = Fn(&mut Whitespace, &mut Context);
