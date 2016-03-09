fn main() {
	let func = |out: i32| { println!("Out: {}", out); };
	let func2 = || { println!("Got none"); };
	let opt: Option<i32> = Some(42);

    // if we want to call function on unwrapped Option value and do nothing if it's None

    // 1. use pattern-matching as usual
    match opt {
    	Some(x) => { func(x); },
    	None => {},
    }

    // 2. use conditions:
    if opt.is_some() {
    	let x = opt.unwrap();
    	func(x);
    }

    // latter can be done in more elegant way:
    if let Some(x) = opt {
    	func(x);
    }

    // or with else
    let opt2: Option<i32> = None;
    if let Some(x) = opt2 { func(x) }
    else { func2(); }

    // while let. Turns code like this:
    let mut v = vec![1, 3, 5, 7, 11];
    loop {
    	match v.pop() {
    	    Some(x) => println!("{}", x),
    	    None => break,
    	}
    }

    // into code like this:
    let mut v = vec![1, 3, 5, 7, 11];
    while let Some(x) = v.pop() {
    	println!("{}", x);
    }
}
