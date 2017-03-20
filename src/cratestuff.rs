pub mod cratemod {
    extern crate rand;
    use rand::Rng;

    pub fn print_random() {
        let mut rng = rand::thread_rng();
        println!("u8: {}, u8: {}", rng.gen::<u8>(), rng.gen::<u8>())
    }
}
