#![a]
#![b]
#![rustidy::config(indent = "\t\t")]

mod a {
	fn a() { a; b; }

	#[rustidy::config(indent = "  ")]
	fn a() { a; b; }

	#[rustidy::config(indent = "/*indent*/")]
	fn a() { a; b; }

	#[rustidy::config(indent = "/*indent*/")]
	#[a]
	#[b]
	fn a() { a; b; }
}

mod b {
	#![a]
	#![b]
	#![rustidy::config(indent = "  ")]

	fn a() { a; b }

	mod c {
		#![rustidy::config(indent = "/*indent*/")]
	}
}
