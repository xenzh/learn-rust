extern crate adder;

// idea is to use this for integration tests

#[test]
fn it_works() {
    assert_eq!(4, adder::add_two(2));
}
