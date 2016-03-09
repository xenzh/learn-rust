fn main() {
    // is a Rust's mechanism for dynamic polymorphism!

    // essentials:
    trait Foo {
    	fn method(&self) -> String;
    }

    impl Foo for u8 {
    	fn method(&self) -> String { format!("u8: {}", *self) }
    }
    impl Foo for String {
    	fn method(&self) -> String { format!("string: {}", *self) }
    }

    // static dispatch (like c++'s static polymorphism through templates)
    fn do_it<T: Foo>(x: T) { println!("{}", x.method()); }
    let x = 5u8;
    let y = "Hello".to_string();
    do_it(x);
    do_it(y);

    // 1. monomorphization: creation of all possible template instantiations and call one of them.
    // 2. inlining is possible: callee is known at compiletime, good for performance.
    // 3. tradeoff: code bloat.
    // 4. Rust has #[inline] and #[inline(always)], need to be used carefully (instuction cache bloat)

    // guideline: static dispatch is generally more preferrable.

    // 1. dynamic dispatch via explicit casting
    fn do_it_d(x: &Foo) { println!("{}", x.method()); }
    let x = 42u8;
    do_it_d(&x as &Foo);

    // 2. dynamic dispatch via coercing
    let y = "Coercing".to_string();
    do_it_d(&y);

    // this way, no code bloat, no inlining, slower virtual fx calls
    // pointers: Rust doesn't default to pointers (like Java),
    // but thy are used for dynamic dispatch since we'll need to know arg size at compile time.
}
