#![a]
#![b]
#![rustidy::config(ident = "\t\t")]

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
		#[a]
		#[b]
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

  mod c {
/*indent*//*indent*/#![rustidy::config(ident = "/*indent*/")]
/*indent*/}
}
