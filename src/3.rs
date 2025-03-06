fn main() {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1..=10);
    println!("Random number between 1 and 10: {}", x);
}
