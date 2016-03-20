fn main() {
    // there are types that are useful to express, but doesn't have a defined size
    // one example is [T] - several T's in sequence, but not clear how many.

    // Restrictions:
    // * only via a pointer. &[T] works, [T] - not
    // * variables and arguments cannot be unsized
    // * _only the last_ struct field can be unsized.
    // * enum variants cannot have unsized data.

    // language allows to write impls for type instead of 'for pointer' or 'for reference'

    // ?Sized
    #[allow(dead_code)]
    struct Foo<T: ?Sized> { f: T, }

    // ?Sized == T may be Sized
}
