mod algorithms;

use algorithms::*;

fn main() {
    println!("Hello, world!");
    println!("Is 17 prime? {}", trial_division::is_prime(17));
}
