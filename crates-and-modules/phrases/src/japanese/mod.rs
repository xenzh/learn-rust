pub use self::greetings::*; // wildcard syntax
pub use self::farewells::goodbye; // brings function into current scope

pub mod greetings;
pub mod farewells;