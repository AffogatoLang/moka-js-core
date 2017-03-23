const foo = (a, b) => a.call ? a(b) : a + b

function double(a) { return 2 * a }

log(foo(foo(double, 123), 4))