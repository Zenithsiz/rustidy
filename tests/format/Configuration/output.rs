#![a]
#![b]
#![rustidy::config(indent = "\t\t")]

mod a {
		fn a() {
				a;
				b;
		}

		#[rustidy::config(indent = "  ")]
		fn a() {
    a;
    b;
  }

		#[rustidy::config(indent = "/*indent*/")]
		fn a() {
/*indent*//*indent*/a;
/*indent*//*indent*/b;
/*indent*/}

		#[rustidy::config(indent = "/*indent*/")]
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
  #![rustidy::config(indent = "  ")]

  fn a() {
    a;
    b
  }

  mod c {
/*indent*//*indent*/#![rustidy::config(indent = "/*indent*/")]
/*indent*/}
}
