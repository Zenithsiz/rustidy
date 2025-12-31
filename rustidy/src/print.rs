//! Formatting

// Exports
pub use rustidy_macros::Print;

// Imports
use {crate::Parser, core::marker::PhantomData, std::fmt};

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

impl<T> Print for PhantomData<T> {
	fn print(&self, _f: &mut PrintFmt) -> Result<(), fmt::Error> {
		Ok(())
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
pub struct PrintFmt<'a, 'input> {
	output: String,
	parser: &'a Parser<'input>,
}

impl<'a, 'input> PrintFmt<'a, 'input> {
	/// Creates a new formatter
	#[must_use]
	pub const fn new(parser: &'a Parser<'input>) -> Self {
		Self {
			output: String::new(),
			parser,
		}
	}

	/// Returns the parser of this formatter
	#[must_use]
	pub const fn parser(&self) -> &'a Parser<'input> {
		self.parser
	}

	/// Returns the output
	#[must_use]
	pub fn output(&self) -> &str {
		&self.output
	}
}

impl fmt::Write for PrintFmt<'_, '_> {
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
