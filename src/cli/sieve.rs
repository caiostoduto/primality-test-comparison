use strum::IntoEnumIterator;

use crate::cli::parsing::SieveAlgorithm;

pub fn handle_cli(number: u64, algorithms: &Option<Vec<SieveAlgorithm>>) {
    // Run benchmark
    if algorithms.is_none() {
        println!("❗️ No algorithm specified. Running all algorithms.");

        for alg in SieveAlgorithm::iter() {
            run_sieve(number, alg);
        }
    } else {
        for alg in algorithms.as_ref().unwrap() {
            run_sieve(number, *alg);
        }
    }
}

fn run_sieve(number: u64, algorithm: SieveAlgorithm) {
    println!(
        "🔍 Testing sieve algorithm '{}' for numbers up to {}...",
        algorithm.as_str(),
        number
    );

    let start_time = std::time::Instant::now();
    let primes = algorithm.as_algorithm_fn()(number);
    let duration = start_time.elapsed();

    println!("\n✅ Result: [0, {}] has {} primes", number, primes.len());
    println!("⏱️  Time taken: {:.4?}", duration);
}
