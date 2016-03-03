fn take(mut v: Vec<i32>) {
    v.push(42)
}

fn foo(v: &Vec<i32>) -> i32 {
    v[0]
}

fn main() {
    // move semantics by default. immutable by default
    let v = vec![1, 2, 3];
    take(v);
    // println!("waaat {:?}", v); // will generate compilation error

    // to change default to copying, implement Copy trait. All fundamentsls do.
    let zz = 42;
    let cpzz = zz;
    println!("both, {}, {}", zz, cpzz);

    // 2. borrowing. like references, allows to use bindling after return
    let v = vec![3, 2, 1];
    println!("hohooo, sequence points! {}, {:?}", foo(&v), v);
}
