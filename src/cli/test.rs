use strum::IntoEnumIterator;

use crate::cli::parsing::PrimeAlgorithm;

pub fn handle_cli(number: u64, algorithms: &Option<Vec<PrimeAlgorithm>>) {
    // Run benchmark
    if algorithms.is_none() {
        println!("❗️ No algorithm specified. Running all algorithms.");

        for alg in PrimeAlgorithm::iter() {
            run_test(number, alg);
        }
    } else {
        for alg in algorithms.as_ref().unwrap() {
            run_test(number, *alg);
        }
    }
}

fn run_test(number: u64, algorithm: PrimeAlgorithm) {
    println!(
        "🔍 Testing if {} is prime using '{}'...",
        number,
        algorithm.as_str()
    );

    let start_time = std::time::Instant::now();
    let is_prime = algorithm.as_algorithm_fn()(number);
    let duration = start_time.elapsed();
    let result_str = if is_prime { "prime" } else { "composite" };

    println!("\n✅ Result: {} is {}", number, result_str);
    println!("⏱️  Time taken: {:.4?}", duration);
}
