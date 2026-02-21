#![a]
#![a::b]
#![a = 5]
#![a = 1 + 2]
#![a()]
#![a(a)]
#![a(a::b)]
#![a(a, b, c, d,)]
#![a(a = 5, b(c, d = 1), h, y = 4)]
#![a(b+1)]

// Not (!) `MetaItem`, but valid
#![a(a b)]
#![a(a = let)]
