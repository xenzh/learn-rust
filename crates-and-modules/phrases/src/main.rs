// basic imports
extern crate phrases;
use phrases::english::{greetings, farewells};
use phrases::japanese;

// complex imports
extern crate phrases as sayings;                   // aliasing
use sayings::japanese::greetings as ja_greetings;  // aliasing
use sayings::japanese::farewells::*;               // glob import

// compressed 3 imports, {} cannot be used with ::*
use sayings::english::{self, greetings as en_greetings, farewells as en_farewells};


fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye()); // use directive

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());

    // from complex imports
}