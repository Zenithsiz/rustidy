union A {}

union A<T> {}
union A where T: B {}

union A { a: u32 }
union A { pub a: u32 }
union A { #[a] a: u32 }
union A { a: u32, b: u32, }

union A<T> where T: B { #[a] pub a: u32, #[b] pub b: u32, }
