fn main() {
    // in Rust, strings are UTF-8 and not null-terminated (can contain \0 bytes)
    // two main types:
    // * &str
    // * String

    // &str - 'string slices' - have fixed size, cannot be mutated
    let _greet = "Hi there"; // _greet: &'static str

    // literals span on several lines in 2 ways:
    let _s = "ola 
        mundo";
    assert_eq!("ola \n        mundo", _s);

    let _s = "ola \
        mundo";
    assert_eq!("ola mundo", _s);

    // String - UTF-8, growable, allocated in heap
    let mut _s = "Hello".to_string();
    println!("{}", _s);
    _s.push_str(", world");
    println!("{}", _s);

    // String converts to the slice (coersion)
    let takes_slice = |slice: &str| { println!("slice is \'{}\'", slice); };
    let _s = "chalk and coal from iamthemorning!".to_string();
    takes_slice(&_s);

    // no direct indexing available!
    // since String contains UTF-8 sequences, not bytes or words

    let hachiko = "忠犬ハチ公";
    for b in hachiko.as_bytes() { println!("{}", b); }
    for c in hachiko.chars() { println!("{}", c); }

    // closest analog to indexing:
    println!("n-th: {}", hachiko.chars().nth(1).unwrap());

    // slicing:
    let dog = "hachiko";
    let _hachi = &dog[0..5]; // byte offsets, not char offsets!

    // concatenation
    let hello = "Hello ".to_string();
    let world = "world!";
    println!("{}", hello + world);

    // but for two strings need explicit '&'
    let hello = "Hello ".to_string();
    let world = "world!".to_string();
    println!("{}", hello + &world);

    // &String can automatically coerce to &str
    // this has something to do w/ deref coersion
}
