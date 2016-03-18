// two possible way to apply:

// 1. applies to the next item
#[test]
struct Foo;

// 2. applies to enclosing scope
mod bar { #![test] }

// test attribute defines unit test case functions
#[test]
fn check() { assert_eq!(2, 1 + 1); }

// have a value
#[inline(always)]
fn super_fast_fn() -> i32 { 42 }

#[cfg(target_os = "windows")]
mod win32_only { }

// see full list in reference:
// https://doc.rust-lang.org/reference.html#attributes

// no user-defined attributes

fn main() {
    println!("Hello, world!");
}
