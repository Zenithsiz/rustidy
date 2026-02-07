mod a {
	fn a() {
		a;
		b;
	}

	#[rustidy::config(ident = "  ")]
	fn a() {
    a;
    b;
  }

	#[rustidy::config(ident = "/*indent*/")]
	fn a() {
/*indent*//*indent*/a;
/*indent*//*indent*/b;
/*indent*/}

	#[rustidy::config(ident = "/*indent*/")]
/*indent*/#[a]
/*indent*/#[b]
	fn a() {
/*indent*//*indent*/a;
/*indent*//*indent*/b;
/*indent*/}
}

mod b {
	#![a]
	#![b]
  #![rustidy::config(ident = "  ")]

  fn a() {
    a;
    b
  }
}
