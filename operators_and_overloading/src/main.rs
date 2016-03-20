use std::ops::Add;
use std::ops::Mul;

fn main() {
    // Rust allows limited operators overloading
    // There are special traits for overloadng operators

    #[derive(Debug)]
    struct Point { x: i32, y: i32, }

    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point { x: self.x + other.x, y: self.y + other.y }
        }
    }

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);

    // not clear why ops trait require to move args

    // using operator overloads is traits for generic structs
    trait HasArea<T> { fn area(&self) -> T; }
    struct Square<T> { _x: T, _y: T, side: T, }

    impl<T> HasArea<T> for Square<T>
        where T: Mul<Output=T> + Copy {
        
        fn area(&self) -> T {
            self.side * self.side // Copy is important here, Rust would try to move self.side to return value then
        }
    }

    let s = Square { _x: 0.0f64, _y: 0.0f64, side: 12.0f64 };
    println!("square (side = {}) area is {}", s.side, s.area());
}
