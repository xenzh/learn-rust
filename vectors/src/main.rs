fn main() {
    // just like plain std::vector<T> from c++.

    // use vec! to create
    let _v = vec![1, 2, 3, 4, 5];
    let _v = vec![0; 10]; // ten zeroes

    // accessing elements
    println!("third element is {}", _v[2]);

    // vectors can be indexed _only_ with usize-typed values
    let i: usize = 3;
    println!("{}", _v[i]);

    // iterating
    let mut _v = vec![1, 2, 3, 4, 5];

    for i in &_v { println!("a reference to {}", i); }
    for i in &mut _v { println!("a mut reference to {}", i); }
    for i in _v { println!("take ownership of vector and element {}", i); }
}
