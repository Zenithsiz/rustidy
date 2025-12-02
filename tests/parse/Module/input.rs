mod a;
unsafe mod a;

mod a {
	mod b;
}

unsafe mod a {
	#![a]
	#![b]
	mod b;
}

mod a {
	#![a]
	#![b]
	mod b {
		#![c]
		#![d]
	}
}
