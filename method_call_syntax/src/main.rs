fn main() {
    struct Circle {
        _x: f64,
        _y: f64,
        radius: f64,
    }

    // many impl blocks are possible
    impl Circle {
        // possible options: &self (default), self (on stack), &mut self
        fn _reference(&self) { println!("takes a reference"); }
        fn _mut_reference(&mut self) { println!("takes mutable reference"); }
        fn _owns(self) { println!("takes the ownership"); }
    }

    // method call syntax
    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    let c = Circle { _x: 0.0, _y: 0.0, radius: 2.0 };
    println!("Circle area = {}", c.area());

    // chained calls
    impl Circle {
        fn grow(&self, increment: f64) -> Circle {
            Circle { _x: self._x, _y: self._y, radius: self.radius + increment }
        }
    }
    println!("It grows! area = {}", c.grow(2.0).area());

    // associated functions
    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle { _x: x, _y: y, radius: radius }
        }
    }
    let _c = Circle::new(0.0, 0.0, 42.0);

    // builder pattern
    struct CircleBuilder {
        x: f64,
        y: f64,
        r: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder { x: 0.0, y: 0.0, r: 1.0 }
        }

        fn make(&mut self) -> Circle {
            Circle { _x: self.x, _y: self.y, radius: self.r }
        }

        fn x(&mut self, coord: f64) -> &mut CircleBuilder { self.x = coord; self }
        fn y(&mut self, coord: f64) -> &mut CircleBuilder { self.y = coord; self }
        fn r(&mut self, coord: f64) -> &mut CircleBuilder { self.r = coord; self }
    }

    let c = CircleBuilder::new().x(1.0).y(2.0).r(5.0).make();
    println!("builder-constructed circle area: {}", c.area());
}
