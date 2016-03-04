use std::fmt::Debug;

fn main() {
    // how it is done
    struct Circle {
        _x: f64,
        _y: f64,
        r: f64,
    }

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.r * self.r)
        }
    }

    let _c = Circle { _x: 1.0, _y: 2.0, r: 3.3 };
    println!("{}", _c.area());

    // and that's how you use it:
    fn print_area<T: HasArea>(shape: &T) { println!("T: HasArea, area = {}", shape.area()); }
    print_area(&_c);

    // and with the following it starts to amke sense
    struct Square { _x: f64, _y: f64, side: f64, };
    impl HasArea for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }
    let _s = Square { _x: 1.0, _y: 2.0, side: 3.3, };
    print_area(&_s);

    // the same trick, but with generic structs
    struct Rectangle<T> { _x: T, _y: T, width: T, height: T, }
    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    let mut rect = Rectangle { _x: 1.0, _y: 2.0, width: 42.0, height: 42.0, };
    assert!(rect.is_square());
    rect.height = 47.0;
    assert!(!rect.is_square());

    // there are operator traits!!

    // can implement traits for std types.
    // considered poor style!
    {
        impl HasArea for i32 {
            fn area(&self) -> f64 {
                println!(">> area() call for i32, again: poor style");
                *self as f64
            }
        }

        println!( "area of '5': {}", 5.area() );
    }

    // Trait rules: #1
    // if trait is not defined in scope, it does not apply
    {
        use std::io::Write; // without it won't compile
        let f = std::fs::File::open("foo.txt");

        if f.is_ok() {
            let buf = b"whatever"; // byte string literal. buf: &[u8, 8]
            let _result = f.unwrap().write(buf);
        } else {
            println!("file error, {}", f.unwrap_err());
        }
    }

    // Trait rules, #2
    // To impl the trait, either it or the type _must_ be in my code.
    // e.g. implementing ToString for i32 will fail.

    // See trail_objects package for more details on generic function with trait bounds.

    // mutliple trait bounds
    fn _foo<T: Clone + Debug>(x: T) {
        x.clone();
        println!("{:?}", x);
    }

    // Another syntax for defining traits:
    fn _bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }
    // or whitespaced:
    fn _bar2<T, K>(x: T, y: K)
        where T: Clone,
              K: Clone + Debug {

        x.clone();
        y.clone();
        println!("{:?}", y);
    }

    // where can be used with types also
    trait ConvertTo<Output> {
        fn convert(&self) -> Output;
    }

    impl ConvertTo<i64> for i32 {
        fn convert(&self) -> i64 { *self as i64 }
    }
    // can be called with i32 (since we defined the trait for it)
    fn _normal<T: ConvertTo<i64>>(x: &T) -> i64 {
        x.convert()
    }
    // can ba called with i64 (since then ConvertTo<T> == ConvertTo<i64>)
    fn _inverse<T>() -> T where i32: ConvertTo<T> {
        42.convert()
    }
    fn _converter<T, X>(t: &T) -> X where T: ConvertTo<X> {
        t.convert() // looks like Rust can deduce generic fn parameter type from return value?
    }

    // default methods
    trait Foo {
        fn is_valid(&self) -> bool;
        fn is_invalid(&self) -> bool { !self.is_valid() }
    }
    struct UseDefault;
    impl Foo for UseDefault {
        fn is_valid(&self) -> bool {
            println!("Called UseDefault.is_valid()");
            true
        }
    }
    // possible to override it:
    struct OverrideDefault;
    impl Foo for OverrideDefault {
        fn is_valid(&self) -> bool {
            println!("Called OverrideDefault.is_valid()");
            true
        }
        fn is_invalid(&self) -> bool {
            println!("Called OverrideDefault.is_invalid()");
            true
        }
    }

    let default = UseDefault;
    assert!(!default.is_invalid());

    let over = OverrideDefault;
    assert!(over.is_invalid());

    // inheritance!
    trait Base { fn foo(&self); }
    trait Descendant : Base { fn foobar(&self); }

    // and implementation for it:
    struct Baz;
    impl Base for Baz { fn foo(&self) { println!("base, foo()"); } }
    impl Descendant for Baz { fn foobar(&self) { println!("descendant, foobar()"); } }

    let bz = Baz;
    bz.foo();
    bz.foobar();

    // deriving
    // for some std traits one can use derive attribute to add automatic implementation

    #[derive(Debug)]
    struct Foox;
    println!("{:?}", Foox); // !!

    // derive-enabled traits:
    // * Clone
    // * Copy
    // * Debug
    // * Eq
    // * Hash
    // * Ord
    // * PartialEq
    // * PartialOrd
}
