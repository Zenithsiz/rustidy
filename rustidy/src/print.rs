//! Formatting

// Exports
pub use rustidy_macros::Print;

// Imports
use {crate::Parser, std::fmt};

/// Printable types
pub trait Print: Sized {
	/// Prints this type onto a writer
	fn print(&self, f: &mut PrintFmt) -> Result<(), fmt::Error>;
}

impl<T: Print> Print for &'_ T {
	fn print(&self, f: &mut PrintFmt) -> Result<(), fmt::Error> {
		(**self).print(f)
	}
}

impl<T: Print> Print for Box<T> {
	fn print(&self, f: &mut PrintFmt) -> Result<(), fmt::Error> {
		(**self).print(f)
	}
}

impl<T: Print> Print for Option<T> {
	fn print(&self, f: &mut PrintFmt) -> Result<(), fmt::Error> {
		if let Some(value) = self {
			value.print(f)?;
		}

		Ok(())
	}
}

impl<T: Print> Print for Vec<T> {
	fn print(&self, f: &mut PrintFmt) -> Result<(), fmt::Error> {
		for value in self {
			value.print(f)?;
		}

		Ok(())
	}
}

impl Print for ! {
	fn print(&self, _f: &mut PrintFmt) -> Result<(), fmt::Error> {
		*self
	}
}

impl Print for () {
	fn print(&self, _f: &mut PrintFmt) -> Result<(), fmt::Error> {
		Ok(())
	}
}

macro tuple_impl($N:literal, $($T:ident),* $(,)?) {
	#[derive(Debug, Print)]
	#[expect(non_snake_case)]
	struct ${concat( Tuple, $N )}< $( $T, )* > {
		$( $T: $T, )*
	}

	#[automatically_derived]
	impl< $($T: Print,)* > Print for ( $($T,)* ) {
		#[expect(non_snake_case)]
		fn print(&self, f: &mut PrintFmt) -> Result<(), fmt::Error> {
			let ( $($T,)* ) = self;
			${concat( Tuple, $N )} { $( $T, )* }.print(f)
		}
	}
}

tuple_impl! { 1, T0 }
tuple_impl! { 2, T0, T1 }
tuple_impl! { 3, T0, T1, T2 }

/// Print formatter
pub struct PrintFmt<'a, 'input, 'output> {
	parser: &'a Parser<'input>,
	output: &'output mut String,
}

impl<'a, 'input, 'output> PrintFmt<'a, 'input, 'output> {
	/// Creates a new formatter from a parser and output string
	pub const fn new(parser: &'a Parser<'input>, output: &'output mut String) -> Self {
		Self { parser, output }
	}

	/// Returns the parser of this formatter
	#[must_use]
	pub const fn parser(&self) -> &'a Parser<'input> {
		self.parser
	}

	/// Splits this formatter into a parser and writer
	#[must_use]
	pub fn split(&mut self) -> (&'a Parser<'input>, &mut impl fmt::Write) {
		(self.parser, self.output)
	}
}

impl fmt::Write for PrintFmt<'_, '_, '_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.output.write_str(s)
	}

	fn write_char(&mut self, c: char) -> fmt::Result {
		self.output.write_char(c)
	}

	fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
		self.output.write_fmt(args)
	}
}
