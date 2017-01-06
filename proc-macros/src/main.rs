// https://cbreeden.github.io/Macros11/

// this feature gate won't be needed starting from 1.15
#![feature(proc_macro)]

#[macro_use]
extern crate hello_world_macro;

#[derive(HelloWorld)]
struct Astroneer;

#[derive(HelloWorld)]
struct NoMansSky;

fn main() {
    Astroneer::hello_world();
    NoMansSky::hello_world();
}