fn main() {
	// that's how to define generics here in Rusr
    enum LikeOption<T> {
    	Some(T),
    	None,
    }
    let _hide_warning: LikeOption<()> = LikeOption::None;

    let _x: LikeOption<i32> = LikeOption::Some(5);
    //let _x: LikeOption<f64> = Some(42); // type mismatch - by default Rust assume i32 for literals
    let _x: LikeOption<f64> = LikeOption::Some(42f64);

    // or even with multiple types:
    enum LikeResult<OneT, TwoT> {
    	Ok(OneT),
    	Err(TwoT),
    }
    let _x: LikeResult<i32, i32> = LikeResult::Ok(5);
    let _x: LikeResult<i32, String> = LikeResult::Err("Generics rock".to_string());

    // generic function:
    fn takes_everything<Ta, Tb>(x: Ta, y: Tb) -> (Ta, Tb) { (x, y) };
    println!("{:?}", takes_everything(42, 5.3));

    // generic structs
    struct Point<T> { x: T, y: T, }
    let _int_point = Point { x: 0, y: 0 };
    let _float_point = Point { x: 0.0, y: 1.0 };

    // impl block as well:
    impl<T> Point<T> {
    	fn swap(&mut self) {
    		std::mem::swap(&mut self.x, &mut self.y);
    	}
    }
    let mut to_swap = Point { x: "iamthemorning".to_string(), y: "chalk and coal".to_string() };
    println!("{}, {}", to_swap.x, to_swap.y);
    to_swap.swap();
    println!("{}, {}", to_swap.x, to_swap.y);
}
