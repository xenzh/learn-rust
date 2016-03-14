fn main() {
    // general syntax:
    let plus_one = |x:i32| x + 1;
    assert_eq!(2, plus_one(1));

    // {} is an expression:
    let plus_two = |x| { let mut res:i32 = x; res += 2; res };
    assert_eq!(2, plus_two(0));

    // return typecan optionally be defined (devs decided not to for ergonomic reasons).
    let plus_one = |x: i32| -> i32 { x + 1 };
    assert_eq!(2, plus_one(1));

    // closures may borrow local bindings:
    let mut _num = 5; // silence 'doesn't need tobe mutable' warning
    let plus_num = |x: i32| x + _num;
    assert_eq!(10, plus_num(5));
    //let y = &mut num; // generates borrow checker error

    // the same but legal since there is a scope boundaries:
    let mut num = 5;
    {
        let _plus_num = |x: i32| x + num;
    }
    let _y = &mut num;

    // Rust may not just borrow, but transfer binding ownership:
    let nums = vec![1, 2, 3, 4, 5];
    let takes_nums = || nums;
    // println!("{:?}", nums); // use of moved value error
    println!("{:?}", takes_nums());

    // move closures
    // normal move semantics: 5 implements Copy, and owns_num takes ownership of a copy of num
    let num = 5;
    let _owns_num = move |x: i32| x + num;

    // now about the difference.
    // here we borrowed num and modified it, modification persists.
    // closure takes a mutable reference to num
    let mut num = 5;
    { let mut add_num = |x: i32| num += x; add_num(5); }
    assert_eq!(10, num);

    // this one doesn't take mut reference, but ownership of a copy, so original is not modified.
    let mut num = 5;
    { let mut add_num = move |x: i32| num += x; add_num(5); }
    assert_eq!(5, num);

    // move closures, it's like thy give a closure its own stack frame.
    // usual closures may be bound to the frame they are created in, but
    // move closures are self-contained.
    // Implication: generally one _cannot_ return non-move closure from a function

    // closures are implemented as trait objects that overload operator()
    // for 3 possible self-s: self, &self and &mut self

    // passing closure as a result, the same as with the trait
    // static dispatch (no heap allocations. static lambdas!)
    // Fn is a trait for a closure
    fn call_with_one<F>(some_closure: F) -> i32
        where F: Fn(i32) -> i32 {

        some_closure(1)
    }
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);

    // dynamic dispatch:
    // &Fn - trait object. &|| - closure type.
    fn call_with_one2(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }
    let answer = call_with_one2(&|x| x + 2);
    assert_eq!(3, answer);

    // function pointer is like a closure w/o environment, can be used instead of it
    fn add_one(i: i32) -> i32 { i + 1 }
    let f = add_one;
    let answer = call_with_one2(&f);
    println!("with fx ptr 1: {}", answer);
    println!("with fx ptr 2: {}", call_with_one2(&add_one));

    // returning closures:
    fn factory() -> Box<Fn(i32) -> i32> { // cannot make a trait object, need to know the size
        let num = 5;
        Box::new(move |x| x + num) // cannot just borrow num (it will outlive context in this case)
    }
    let f = factory();
    let answer = f(1);
    assert_eq!(6, answer);
}
