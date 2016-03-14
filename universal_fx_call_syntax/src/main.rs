fn main() {
    // functions can have the same names.
    trait Foo { fn f(&self); }
    trait Bar { fn f(&self); }

    #[derive(Debug)]
    struct Baz;
    impl Foo for Baz { fn f(&self) { println!("Foo for: {:?}", *self); } }
    impl Bar for Baz { fn f(&self) { println!("Bar for: {:?}", *self); } }

    let b = Baz;
    //b.f(); // error: multiple applicable errors in scope

    // to call, use 'universal function call syntax':
    Foo::f(&b);
    Bar::f(&b);

    // it was a short-hand, expanded one:
    <Baz as Foo>::f(&b);
    <Baz as Bar>::f(&b);

    // <>:: - syntax for providing a type hint
    // inside <>: 1. Type (Baz). required part
    // inside <>: 2. as Trait (as Foo). optional part, if not ambiguous.
    // <> - optional as well. Type::method() works, too. (as above).

    // example for longer form:
    trait Foox { fn foo() -> i32; }
    struct Barx;
    impl Barx { fn foo() -> i32 { 20 } }
    impl Foox for Barx { fn foo() -> i32 { 40 } }

    println!("bar as foo: {}", <Barx as Foox>::foo());
    println!("bar::foo(): {}", Barx::foo());
}
