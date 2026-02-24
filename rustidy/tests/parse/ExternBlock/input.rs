extern {}

unsafe extern {}
extern "C" {}

extern { #![a] }

extern { a! {} }

extern { static A: u32; }
extern { pub static A: u32; }

extern { fn a(); }
extern { pub fn a(); }

unsafe extern "C" { #![a] a! {} pub static A: u32; pub fn a(); }
