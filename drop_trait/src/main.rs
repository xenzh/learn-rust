fn main() {
    
    // drop trait is a destructor
    struct HasDrop;
    impl Drop for HasDrop {
        fn drop(&mut self) { println!("Dropping!"); }
    }
    { let _x = HasDrop; }

    // values are destructed in an order, opposite to declaration
    struct Firework { strength: i32, }
    impl Drop for Firework { fn drop(&mut self) { println!("boom x{}", self.strength); } }
    {
        let _firecracker = Firework { strength: 1 };
        let _tnt = Firework { strength: 100 };
    }
}
