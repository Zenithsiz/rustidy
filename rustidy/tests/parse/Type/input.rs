type A = (T);

type A = impl A;
type A = dyn ?A;
type A = ?A;

type A = A;
type A = ::A;
type A = A::B;
type A = A<B, C,>;
type A = A(A, B) -> C;
type A = A::<B, C,>;
type A = A::(A, B) -> C;
type A = super;
type A = self;
type A = Self;
type A = crate;
type A = $crate;

type A = ::A::(B, C) -> u32::C;

type A = ();
type A = (A,);
type A = (A, B);

type A = !;

type A = *const A;
type A = *mut A;

type A = &A;
type A = &'a A;
type A = &'a mut A;

type A = [A; 5];

type A = [A];

type A = _;

type A = <A>::B;
type A = <A as B>::C;
type A = <A as B>::C::D;

type A = fn();
type A = for<'a> fn();
type A = unsafe fn();
type A = extern fn();
type A = extern "C" fn();
type A = fn(u32);
type A = fn(a: u32);
type A = fn(_: u32);
type A = fn(#[a] a: u32);
type A = fn(u32, ...);
type A = fn(u32, #[a] ...);
type A = fn() -> u32;

type A = for<'a> unsafe extern "C" fn(#[a] a: u32, #[b] c: u32,) -> u32;
type A = for<'a> unsafe extern "C" fn(#[a] a: u32, #[b] c: u32, #[c] ...) -> u32;

type A = a!{};


type A = impl A + B;
type A = dyn A + B;
type A = A + B;
