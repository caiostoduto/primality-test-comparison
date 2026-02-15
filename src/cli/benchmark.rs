use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use strum::IntoEnumIterator;

use crate::cli::parsing::PrimeAlgorithm;

struct PrimeResultLocal {
    number: u64,
    timestamp: std::time::SystemTime,
}

struct PrimeResultFinal {
    number: u64,
    elapsed: u64, // Elapsed time in microseconds since benchmark start
    thread_id: usize,
}

pub fn handle_cli(
    duration_str: &str,
    algorithms: &Option<Vec<PrimeAlgorithm>>,
    output_path: &PathBuf,
    save: &bool,
) {
    // Run benchmark
    if algorithms.is_none() {
        println!("❗️ No algorithm specified. Running all algorithms.");

        for alg in PrimeAlgorithm::iter() {
            run_benchmark(duration_str, alg, output_path, save);
        }
    } else {
        for alg in algorithms.as_ref().unwrap() {
            run_benchmark(duration_str, *alg, output_path, save);
        }
    }
}

fn run_benchmark(
    duration_str: &str,
    algorithm: PrimeAlgorithm,
    output_path: &PathBuf,
    save: &bool,
) {
    // Parse duration
    let duration = parse_duration(duration_str).unwrap_or_else(|e| {
        eprintln!("⚠️ Error parsing duration '{}': {}", duration_str, e);
        eprintln!("Valid formats: 5s, 10m, 1h, 30sec, 2min, etc.");
        std::process::exit(1);
    });

    println!(
        "⏱️  Running '{}' benchmark for {:?}...",
        algorithm.as_str(),
        duration
    );

    // Shared state for tracking primes across all threads
    let running = Arc::new(AtomicBool::new(true));
    let primes_vector = Arc::new(std::sync::Mutex::new(Vec::<PrimeResultFinal>::new()));

    // Setup timer thread
    let running_clone = running.clone();
    thread::spawn(move || {
        thread::sleep(duration);
        println!("\n⏰ Time's up! Stopping all threads...");
        running_clone.store(false, Ordering::SeqCst);
    });

    // Run the primality test in parallel
    let handles = is_prime_in_parallel(
        algorithm.as_algorithm_fn(),
        running.clone(),
        primes_vector.clone(),
    );

    for handle in handles {
        handle.join().unwrap();
    }

    // Print final results
    let final_count = primes_vector.lock().unwrap().len();
    println!("\n📊 Final Results:");
    println!("   Primes found: {}", final_count);

    if !*save {
        return;
    }

    // Order primes by timestamp
    let mut primes = primes_vector.lock().unwrap();
    primes.sort_by_key(|p| p.elapsed);

    // Create output directory if it doesn't exist
    let _ = fs::create_dir(output_path);

    // Generate readable timestamp for filename
    let filename = format!(
        "{}/{}-{}.parquet",
        output_path.display(),
        algorithm.as_str(),
        duration_str
    );

    // Write results to Parquet file
    write_to_parquet(&filename, &primes).unwrap();
    println!("\n💾 Results written to: {}", filename);
}

fn write_to_parquet(
    filename: &str,
    primes: &[PrimeResultFinal],
) -> Result<(), Box<dyn std::error::Error>> {
    use arrow::array::{ArrayRef, UInt64Builder};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::fs::File;

    // Define schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("elapsed", DataType::UInt64, false),
        Field::new("thread", DataType::UInt64, false),
        Field::new("number", DataType::UInt64, false),
    ]));

    // Create arrays for each column
    let mut elapsed_builder = UInt64Builder::new();
    let mut thread_builder = UInt64Builder::new();
    let mut number_builder = UInt64Builder::new();

    for prime in primes {
        let ts = prime.elapsed as u64;
        elapsed_builder.append_value(ts);
        thread_builder.append_value(prime.thread_id as u64);
        number_builder.append_value(prime.number);
    }

    // Build the record batch
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(elapsed_builder.finish()) as ArrayRef,
            Arc::new(thread_builder.finish()) as ArrayRef,
            Arc::new(number_builder.finish()) as ArrayRef,
        ],
    )?;

    // Write to parquet file
    let file = File::create(filename)?;
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    humantime::parse_duration(s).map_err(|e| e.to_string())
}

fn is_prime_in_parallel(
    primality_test_func: fn(u64) -> bool,
    running: Arc<AtomicBool>,
    primes_vector: Arc<std::sync::Mutex<Vec<PrimeResultFinal>>>,
) -> Vec<thread::JoinHandle<()>> {
    // Amount of threads to spawn based on available parallelism
    let parallelism_count = thread::available_parallelism().unwrap().get();
    // Thread handles
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    // Start time for calculating elapsed time for each prime found
    let start_time = std::time::SystemTime::now();

    // Spawn threads
    for i in 0..parallelism_count {
        // Clone shared state for each thread
        let running = running.clone();
        let primes_vector = primes_vector.clone();

        // Each thread will have its own local vector to store primes before pushing to shared vector
        let handle = thread::spawn(move || {
            // Local vector for this thread to store primes before pushing to shared vector
            let mut local_primes: Vec<PrimeResultLocal> = Vec::new();

            // Special handling for the first thread to check small primes
            if i == 0 {
                // Check small primes (2 and 3) before entering the main loop
                for k in 2..3 {
                    // Check if we should stop
                    if !running.load(Ordering::SeqCst) {
                        // Append local primes to shared primes vector before exiting
                        let mut shared_primes = primes_vector.lock().unwrap();
                        for p in local_primes.drain(..) {
                            shared_primes.push(PrimeResultFinal {
                                number: p.number,
                                elapsed: p.timestamp.duration_since(start_time).unwrap().as_micros()
                                    as u64,
                                thread_id: i,
                            });
                        }
                        return;
                    }

                    // Check if k is prime and add to local primes if it is
                    if primality_test_func(k) {
                        local_primes.push(PrimeResultLocal {
                            number: k - 1,
                            timestamp: std::time::SystemTime::now(),
                        });
                    }
                }
            }

            for j in 0.. {
                // Check if we should stop
                if !running.load(Ordering::SeqCst) {
                    let count = local_primes.len();

                    // Append local primes to shared primes vector before exiting
                    let mut shared_primes = primes_vector.lock().unwrap();
                    for p in local_primes.drain(..) {
                        shared_primes.push(PrimeResultFinal {
                            number: p.number,
                            elapsed: p.timestamp.duration_since(start_time).unwrap().as_micros()
                                as u64,
                            thread_id: i,
                        });
                    }

                    println!("Thread {} stopping... (count: {})", i, count);
                    return;
                }

                // Calculate candidate k based on thread index and iteration
                let k: u64 = ((i + 1) * 6 + j * parallelism_count * 6)
                    .try_into()
                    .unwrap();

                // Check candidates k-1 and k+1 (since all primes > 3 are of the form 6k ± 1)
                for candidate in [k - 1, k + 1] {
                    // Check if candidate is prime and add to local primes if it is
                    if primality_test_func(candidate) {
                        local_primes.push(PrimeResultLocal {
                            number: candidate,
                            timestamp: std::time::SystemTime::now(),
                        });
                    }
                }
            }
        });

        // Store thread handle
        handles.push(handle);
    }

    handles
}
