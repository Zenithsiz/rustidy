struct A;
struct A {}
struct A();

struct A<T> {}
struct A where T: B {}

struct A { a: u32 }
struct A { pub a: u32 }
struct A { #[a] a: u32 }
struct A { a: u32, b: u32, }

struct A<T> where T: B { #[a] pub a: u32, #[b] pub b: u32, }

struct A(pub u32, pub u32,);

struct A<T>();
struct A() where T: B;

struct A(u32);
struct A(pub u32);
struct A(#[a] u32);
struct A(u32, u32,);

struct A<T>(#[a] pub u32, #[b] pub u32,) where T: B;
