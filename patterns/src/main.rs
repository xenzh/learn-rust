fn main() {

    // basic pattern matching
    let x = 1;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("other"),
    }

    // pstterns introduce shadowing
    let x = 'x';
    let c = 'c';
    match c {
        x => println!("x: {}, c: {}", x, c), // x: c, c: c  -- original x was shadowed
    }
    println!("x: {}", x);

    // multimatch
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        _ => println!("something else"),
    }

    // destructuring
    // works on any compound data type like enum, struct or tuple
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, y } => println!("({}, {})", x, y),
    }
    // with different aliases
    match origin {
        Point { x: x1, y: y1 } => println!("({}, {})", x1, y1),
    }
    // omit values that are not needed
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    match origin {
        Point { y, .. } => println!("y is {}", y),
    }

    // ignore bindings using "_" or ".."
    let x: Result<Point, &str> = Result::Ok(origin);
    match x {
        Ok(point) => println!("Ok, point.x is {}", point.x),
        Err(_) => println!("error, but we ignore its info"),
    }
    // or in other context:
    let fnx = || { (1, 2, 3) };
    let (_one, _, _three) = fnx();
    // disredard multiple values
    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }
    let x = OptionalTuple::Value(42, 41, 0);
    let _y = OptionalTuple::Missing; // to hush "unused variant" checker
    match x {
        OptionalTuple::Value(..) => println!("Got an actual tuple!"),
        OptionalTuple::Missing => println!("got nothing"),
    }

    // use references of mut references inside the match:
    let x = 5;
    match x {
        ref r => println!("Got a reference to {}", r),
    }
    // or bind the mutable:
    let mut x = 6;
    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    // there are even range matches
    // mostly for ints and chars (like 'a'...'k')
    let x = 1;
     match x {
        1 ... 5 => println!("1..5"),
        _ => println!("..or not"),
     }

    // possible to give left-hand match value a name
    let x = 42;
    match x {
        e @ 1...43 => println!("got a range element, {}", e),
        _ => println!("never mind"),
    }
    // useful for getting a handle of something from a complicated compound type
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }
    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name : ref aa @ Some(_), .. }) => println!("{:?}", aa),
        _ => {},
    }
    // when @ is used with |, all variants should be bound to the same name

    // guards aka additional match conditions
    enum OptionalInt { Value(i32), Missing, }
    let x = OptionalInt::Value(42);
    let _y = OptionalInt::Missing;
    match x {
        OptionalInt::Value(i) if i > 43 => println!("Got an int > 43"),
        OptionalInt::Value(..)          => println!("Got an int <= 43"),
        OptionalInt::Missing            => {}
    }
    // if used with '|', is applied to both branches
    let x = 4;
    let y = false;
    match x {
        4 | 5 if y => println!("4 | (5 if y) -- doesn't work this way"),
        _ => println!("(4 | 5) if y -- that's how the things are"),
    }
}
