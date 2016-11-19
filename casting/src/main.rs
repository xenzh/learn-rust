use std::mem; // for transmute

fn main() {
    // two ways of casting:
    // * 'as' - safe way
    // * 'transmute' - arbitrary casting, as dangerous as reinterpret_cast

    // Coercion - ijplicit casts. No syntax, but can be spelled with 'as'
    // occurs in 'let', 'const', 'static', fn call args, field values in struct init, fn result
    // most common coercions:
    // * &mut T to &T -- removing mutability from a reference
    // * *mut T to *const T -- removing mutability from raw pointers
    // * &T to *const T -- and
    // * &mut T to *mut T -- references are coerced to raw pointers (same mutability)

    // Deref trait can be used to define constom coercions (c++ operator type())
    // coercions are transitive

    // 'as'
    let x: i32 = 5;
    let y = x as i64;
    println!("x:i32 as i64: {}", y);

    // safe cast categories:
    // * explicit coercions
    // * numberic types casts
    // * pointer casts

    // casts are _not_ transitive:
    // if (e as U1 as U2) is valid, (e as U2) isn't necessarily (valid only if U1 coerces to U2)

    // explicit coercions
    // (e as U) valid if (e: T) and T coerces to U

    // numeric casts: (e: T as U) is valid if:
    // * numeric-cast: T and U are any numeric types
    // * enum-cast: T is a C-like enum (w/o data attached to variants), U is an int number
    // * prim-int-cast: T is bool/char, U is an int type.
    // * u8-char-cast: T is u8 and U is char.
    let _one = true as u8;
    let _at_sign = 64 as char;
    let _two_hundred = -56i8 as u8;

    // Numeric cast semantics:
    // * cast between 2 ints of the same size (i32, u32) is a no-op (??? but works)
    // * cast from larger int to smaller int (u32 -> u8) truncates
    // * cast from smaller to lerger int will
    //   * zero-extent if unsigned
    //   * sign-extend if signed
    // * cast from a float to int type will round the float twoards zero
    //   !!! UB if float is Inf/NaN! is a fig, going to be fixed at some point
    // * int -> float -- will round if necessary (unspecific how)
    // * f32 -> f64 -- ok
    // * f64 -> f32 -- closest possible value (rounding unspecified)
    //   !!! UB if value is finite but > or < than largest/smallest value representable by f32. Bug.

    // Pointer casts
    // ok to cast raw ptrs <-> integers, and between raw ptrs of different types (w/ constrants)
    // unsafe is to dereference pointers:
    let a = 300 as *const char; // ptr to 300 location
    let _b = a as u32;

    // (e as U) is a valid cast in following cases:
    // * ptr-ptr-cast: e is *T, U is *U_0, and (U_0: Sized) or (unsize_kind(T) == unsize_kind(U_0))
    // * ptr-addr-cast: e is *T and U is numeric type while (T: Sized)
    // * addr-ptr cast: e is integer and U is *U_0 while (U_0: Sized)
    // * array-ptr-cast: e is &[T; n] and U is *const T
    // * fptr-ptr cast: e is fn ptr type and U is *T where (T: Sized)
    // * fptr-add-cast: e is fn ptr type and U is an int.

    // 'transmute'
    let a = [0u8, 0u8, 0u8, 0u8];
    //let b = a as u32; // generates non-scalar cast error

    unsafe {
        let b = mem::transmute::<[u8; 4], u32>(a);
        println!("transmute: {:?} to u32: {}", a, b);
    }

    // transmute only checks if types have ythe same size.


    // "Deref" coercions
    
}
