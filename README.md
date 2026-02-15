# Primality Test Comparison

![example workflow](https://github.com/caiostoduto/primality-test-comparison/actions/workflows/ci.yml/badge.svg)

A high-performance command-line tool for comparing different primality testing algorithms, written in Rust. This project implements multiple classical and modern algorithms for testing primality, from simple trial division to the sophisticated AKS (Agrawal-Kayal-Saxena) algorithm.

## 🎯 Objective

The primary goal of this project is to provide a practical comparison framework for various primality testing algorithms, allowing developers and mathematicians to:

- **Compare performance** across different algorithmic approaches
- **Benchmark algorithms** under real-world conditions with parallel execution
- **Analyze trade-offs** between speed, complexity, and determinism
- **Generate datasets** for further analysis in Parquet format

This tool is ideal for educational purposes, algorithm analysis, and performance optimization research.

## ✨ Features

- **Multiple Algorithms**: Implements 5 primality testing algorithms and 1 sieve algorithm
- **Parallel Benchmarking**: Multi-threaded benchmark mode to stress-test algorithms
- **Data Export**: Save benchmark results to Parquet files for analysis
- **Flexible CLI**: Easy-to-use command-line interface with multiple operation modes
- **Performance Metrics**: Detailed timing and statistics for each algorithm

## 📦 Installation

### Prerequisites

- Rust 1.70+ (uses 2024 edition)
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/primality-test-comparison.git
cd primality-test-comparison

# Build the project
cargo build --release

# Run the binary
./target/release/primality-test-comparison --help
```

Or run directly with Cargo:

```bash
cargo run --release --help
```

## 🚀 Usage

The tool provides three main commands: `test`, `benchmark`, and `sieve`.

### Command Overview

```
primality-test-comparison <COMMAND>

Commands:
  test       Test if a specific number is prime
  benchmark  Run performance benchmarks on algorithms
  sieve      Generate all primes up to a given number
  help       Print this message or the help of the given subcommand(s)
```

### Test Command

Test if a specific number is prime using one or more algorithms.

```bash
# Test a single number with all algorithms
primality-test-comparison test 982451653

# Test with specific algorithms
primality-test-comparison test 982451653 miller-rabin,trial-division

# Test with the AKS algorithm
primality-test-comparison test 97 aks
```

**Example Output:**
```
🔍 Testing if 982451653 is prime using 'miller-rabin'...

✅ Result: 982451653 is prime
⏱️  Time taken: 23.7910µs
```

### Benchmark Command

Run continuous benchmarking for a specified duration, finding as many primes as possible.

```bash
# Run Miller-Rabin benchmark for 30 seconds
primality-test-comparison benchmark 30s miller-rabin

# Run all algorithms for 5 minutes and save results
primality-test-comparison benchmark 5m --save

# Run with custom output path
primality-test-comparison benchmark 1h trial-division --save -o ./results

# Multiple algorithms at once
primality-test-comparison benchmark 10s miller-rabin,trial-division --save
```

**Duration formats:** `5s`, `10m`, `1h`, `30sec`, `2min`, etc.

**Example Output:**
```
⏱️  Running 'miller-rabin' benchmark for 30s...

⏰ Time's up! Stopping all threads...
Thread 4 stopping... (count: 5846608)
Thread 0 stopping... (count: 5857718)
Thread 6 stopping... (count: 5843578)
Thread 2 stopping... (count: 5895048)
Thread 7 stopping... (count: 5794422)
Thread 3 stopping... (count: 5820421)
Thread 5 stopping... (count: 5849771)
Thread 1 stopping... (count: 5809368)

📊 Final Results:
   Primes found: 46716934
   Biggest prime found: 923705201

💾 Results written to: ./out/miller-rabin-30s.parquet
```

The benchmark runs in parallel across all CPU cores, continuously testing sequential numbers for primality until the time limit is reached.

### Sieve Command

Generate all prime numbers up to a given limit using sieve algorithms.

```bash
# Find all primes up to 1 million
primality-test-comparison sieve 1000000

# Use specific sieve algorithm
primality-test-comparison sieve 1000000 sieve-of-eratosthenes
```

**Example Output:**
```
🔍 Testing sieve algorithm 'sieve-of-eratosthenes' for numbers up to 1000000...

✅ Result: [0, 1000000] has 78498 primes
⏱️  Time taken: 6.3608ms
```

## 🧮 Implemented Algorithms

### Primality Testing Algorithms

#### 1. **Trial Division** (`trial-division`)
- **Type:** Deterministic
- **Complexity:** O(√n)
- **Description:** The most straightforward approach. Tests divisibility by all odd numbers from 3 to √n using multiplication for square comparison (`i * i <= n`).
- **Best for:** Small numbers, educational purposes
- **Note:** Slower for large numbers but simple and reliable

#### 2. **Trial Division with sqrt()** (`trial-division-sqrt`)
- **Type:** Deterministic
- **Complexity:** O(√n)
- **Description:** Similar to basic trial division but pre-computes the square root using floating-point `sqrt()`.
- **Best for:** Comparing performance impact of pre-computation vs inline multiplication
- **Trade-off:** Single sqrt() call vs repeated multiplications

#### 3. **Trial Division with Newton's Method** (`trial-division-newton`)
- **Type:** Deterministic
- **Complexity:** O(√n)
- **Description:** Uses Newton's integer square root method instead of floating-point arithmetic.
- **Best for:** Avoiding floating-point operations, guaranteed integer precision
- **Trade-off:** More complex but potentially faster for very large numbers

#### 4. **Miller-Rabin** (`miller-rabin`)
- **Type:** Deterministic (for u64 range)
- **Complexity:** O(k log³ n)
- **Description:** A probabilistic primality test made deterministic by using a proven set of 12 witnesses that guarantees correctness for all 64-bit integers. Based on Fermat's Little Theorem and strong pseudoprime testing.
- **Best for:** Large numbers where trial division becomes impractical
- **Witnesses used:** [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
- **Note:** Significantly faster than trial division for large primes

#### 5. **AKS (Agrawal-Kayal-Saxena)** (`aks`)
- **Type:** Deterministic
- **Complexity:** O(log⁶ n) (polynomial time)
- **Description:** The first deterministic polynomial-time primality test, proved in 2002. Uses polynomial congruence testing over finite fields.
- **Best for:** Theoretical interest, guaranteed polynomial-time complexity
- **Note:** Slower in practice than Miller-Rabin despite better asymptotic complexity
- **Historical significance:** First proven polynomial-time deterministic primality test

### Sieve Algorithms

#### 1. **Sieve of Eratosthenes** (`sieve-of-eratosthenes`)
- **Type:** Deterministic
- **Complexity:** O(n log log n)
- **Description:** Ancient algorithm for finding all primes up to a limit. Uses bit-packed storage (64 bits per u64) for memory efficiency.
- **Best for:** Finding all primes in a range
- **Memory optimization:** Uses bitwise operations to reduce memory by 8x

## 📊 Benchmark Output Format

When using the `--save` flag, benchmark results are saved in Apache Parquet format with the following schema:

| Column  | Type   | Description                                    |
|---------|--------|------------------------------------------------|
| elapsed | UInt64 | Microseconds since benchmark start             |
| thread  | UInt64 | Thread ID that found the prime                 |
| number  | UInt64 | The prime number that was discovered           |

This format allows for efficient analysis using tools like:
- **Python**: pandas, polars, pyarrow
- **R**: arrow package
- **DuckDB**: Direct Parquet querying
- **Apache Spark**: Distributed processing

## 📝 Examples

### Compare Algorithm Performance

```bash
# Test the same number with all algorithms
cargo run --release test 18446744073709551557 \
  trial-division,miller-rabin,aks
```

### Stress Test for 1 Hour

```bash
# Find as many primes as possible in 1 hour
cargo run --release benchmark 1h miller-rabin --save -o ./results
```

### Quick Sieve Comparison

```bash
# Generate first million primes
cargo run --release sieve 1000000
```

## 🔬 Performance Notes

### Algorithm Selection Guide

- **For numbers < 50,000,000**: Trial division sqrt are fast and simple
- **For numbers > 50,000,000**: Miller-Rabin is significantly faster
- **For finding all primes in range**: Use Sieve of Eratosthenes

### Benchmark Characteristics

- Benchmarks run on all available CPU cores
- Each thread independently searches for primes
- Results are thread-safe and collected in real-time
- Timestamps allow for throughput analysis over time

## 🛠️ Development

### Project Structure

```
src/
├── main.rs                          # Entry point
├── algorithms/                      # Algorithm implementations
│   ├── mod.rs                       # Module exports
│   ├── aks.rs                       # AKS algorithm
│   ├── miller_rabin.rs              # Miller-Rabin test
│   ├── trial_division.rs            # Basic i*i < n trial division
│   ├── trial_division_sqrt.rs       # With i < sqrt(n) pre-computation
│   ├── trial_division_newton.rs     # With Newton square root
│   └── sieve_of_eratosthenes.rs     # Sieve algorithm
└── cli/                             # CLI handling
    ├── mod.rs                       # Module exports
    ├── parsing.rs                   # Argument parsing
    ├── benchmark.rs                 # Benchmark command
    ├── test.rs                      # Test command
    └── sieve.rs                     # Sieve command
```

## 📄 License

This project is licensed under the GNU GPLv3 License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Some ideas for improvements:

- Add more primality testing algorithms (Baillie-PSW, Solovay-Strassen, etc.)
- Optimize existing implementations
- Add visualization tools for benchmark results
- Implement probabilistic vs deterministic mode toggles
- Add GPU acceleration support

## 📚 References

- [Miller-Rabin Primality Test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
- [AKS Primality Test](https://en.wikipedia.org/wiki/AKS_primality_test)
- [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
- [Prime Number Theorem](https://en.wikipedia.org/wiki/Prime_number_theorem)

---

**Made with ❤️ and Rust**
