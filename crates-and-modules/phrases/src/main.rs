extern crate phrases;

use phrases::english;
use phrases::japanese::{greetings, farewells}; // shortcut

fn main() {
    println!("Hello in English: {}", phrases::english::greetings::hello());
    println!("Goodbye in English: {}", english::farewells::goodbye()); // use directive

    println!("Hello in Japanese: {}", greetings::hello());
    println!("Goodbye in Japanese: {}", farewells::goodbye());
}