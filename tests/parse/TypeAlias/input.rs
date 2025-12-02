type A;

type A<T>;
type A: B;
type A where T: B;

type A = B;
type A = B where T: C;

type A<T>: B where T: C = D where T: E;
