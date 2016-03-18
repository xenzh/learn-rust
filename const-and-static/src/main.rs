fn main() {
    // constants in Rust are:
    // * static lifetime
    // * must be annotated with the type;
    // * are efficiently inlined to each use place
    // * references to them might not be equal (becauseof that)
    const _NUM: i32 = 5;

    // statics are like global variables, and:
    // * static lifetime
    // * must be annotated with the type
    static _NUM2: i32 = 5;
    // * references have static lifetimes
    static _NAME: &'static str = "Steve";
    // * can be mutable
    static mut NUM3: i32 = 5;
    //NUM3 = 42; // an error!

    // since it is global and mutable, may cause a rece condition when two threads are accessing it.
    // therefore assignment can only be done in an unsafe block:
    unsafe { NUM3 += 1; println!("{}", NUM3); }

    // Also, any type stored in static must be Sync and may not have Drop trait implementation

    // both const and static must be initialized with constant (since thy are compile-time)
    // const is strongly preferred over static
}
