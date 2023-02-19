use rand;
use rand::prelude::*;

fn main() {
    let seed: [u8; 32] = [1; 32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    for _ in 0..10 {
        println!("{}", rng.gen_range(10..=5000));
    }
}
