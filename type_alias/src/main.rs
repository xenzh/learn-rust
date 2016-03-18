use std::result;

fn main() {
	// typedefs!
	type MyType = String;
	let x: MyType = "Hello".to_string();
    println!("Hello, world!, -- {}", x);

    // like c++ typedefs, typedef is not strong (doesn't create a new type):
    type Num = i32;
    let x: i32 = 5;
    let y: Num = 5;
    println!("i32 vs Num, {}", x == y); // no error, type is the same

    // use tuple struct to get a new type.(newtype pattern)
    struct NewNum(i32);
    let numm = NewNum(42);
    let NewNum(answer) = numm;
    println!("{}", answer);

    // generics too, just like c++11 using:
    enum ErrorType {
    	MaximWatchesSomething,
    	DenisIsSleeping,
    }
    type Result<T> = result::Result<T, ErrorType>;
    let _x: Result<i32> = Err(ErrorType::MaximWatchesSomething);

    // the last one - specializing the Err - is a common practice
}
