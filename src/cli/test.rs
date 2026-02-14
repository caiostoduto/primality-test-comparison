use crate::cli::parsing::Algorithm;

pub fn handle_cli(number: u64, algorithm: &Option<Algorithm>) {
    // Run benchmark
    if algorithm.is_none() {
        println!("❗️ No algorithm specified. Running all algorithms.");

        for alg in &[Algorithm::TrialDivision] {
            run_test(number, *alg);
        }
    } else {
        run_test(number, (*algorithm).unwrap());
    }
}

fn run_test(number: u64, algorithm: Algorithm) {
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
