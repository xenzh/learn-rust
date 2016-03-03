fn main() {

    // basics
    // iterator-based loop
    for x in 0..10 {
        println!("{}", x);
    }

    // ..is equal to
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            }
            None => { break }
        }
    }

    // bad and suboptimal code in Rust
    let nums = vec![1, 2, 3];
    for i in 0..nums.len() {
        println!("{}", nums[i]); // unnecessary biund checking
    }

    // better and more expressive way
    for num in &nums {
        println!("{}", num);
    }

    // consumers
    // produce value or values on an iterator
    // ::<> provides a hint where to collect, _ - placeholder for auto type inference
    let one_2_hundred = (1..101).collect::<Vec<_>>();

    // find takes a closure that takes a reference and returns first match
    let greater_than_42 = (0..100).find(|x| *x > 42);
    match greater_than_42 {
        Some(_) => println!("Found a match"),
        None    => println!("No match found"),
    };

    let sum = (1..4).fold(0, |sum, x| sum + x);
    println!("Sum of 1..4 is {}", sum);

    // iterators, laziness
    let iter = 1..20; // nothing gets generated here
    let accum: u64 = iter.fold(1, |accum, num| accum * num); // consumer makes iterators generate sequence
    println!("Mul of 1..20 is {}", accum);

    // iter() converts a vector to a simple iterator.
    for num in nums.iter() {
        println!("iter(): {}", num);
    }

    // iterator adapters
    // used to produce an iterator from other iterator.

    // this makes an iter of [2..10] from [1..10)
    let new_iter = (1..10).map(|x| x + 1);
    for num in new_iter {
        println!("map : {}", num);
    }

    // inifinite iterator + adapter that returns n elements from the start
    for i in (1..).take(3) {
        println!("take(3): {}", i);
    }

    // filter() - takes a closure, produces an iter closure returns true for.
    for i in (1..10).filter(|&x| x % 2 == 0) { // note filter doesn't consume elements! just takes by reference
        println!("filter even: {}", i);
    }

    // all these things can be chained together:
    for woohoo in (1..)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5) {
        println!("chained: {}", woohoo);
    }
}
