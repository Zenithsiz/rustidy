#![a]
#![b]
#![rustidy::config(ident = "\t\t")]

mod a {
	fn a() { a; b; }

	#[rustidy::config(ident = "  ")]
	fn a() { a; b; }

	#[rustidy::config(ident = "/*indent*/")]
	fn a() { a; b; }

	#[rustidy::config(ident = "/*indent*/")]
	#[a]
	#[b]
	fn a() { a; b; }
}

mod b {
	#![a]
	#![b]
	#![rustidy::config(ident = "  ")]

	fn a() { a; b }
}
