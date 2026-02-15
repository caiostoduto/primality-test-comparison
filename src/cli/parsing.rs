use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use strum_macros::EnumIter;

use crate::algorithms::*;

#[derive(Parser)]
#[command(name = "Primality Test Comparison")]
#[command(version = "1.0.0")]
#[command(about = "A tool to compare different primality testing algorithms", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Benchmark {
        /// Duration to run the benchmark (e.g., 5s, 10m, 1h)
        duration: String,

        /// Algorithm to use (comma-separated, e.g., trial-division,miller-rabin)
        #[arg(value_enum, value_delimiter = ',')]
        algorithms: Option<Vec<PrimeAlgorithm>>,

        /// Sets a custom output folder for the results (default: ./out)
        #[arg(short, long, value_name = "FOLDER", default_value = "./out")]
        output_path: PathBuf,

        /// Save results to a file (default: false)
        #[arg(short, long, default_value = "false")]
        save: bool,
    },
    Test {
        /// Number to test for primality
        number: u64,

        /// Algorithm to use (comma-separated, e.g., trial-division,miller-rabin)
        #[arg(value_enum, value_delimiter = ',')]
        algorithms: Option<Vec<PrimeAlgorithm>>,
    },
    Sieve {
        /// Number to generate primes up to
        number: u64,

        /// Algorithm to use (comma-separated, e.g., sieve-of-eratosthenes)
        #[arg(value_enum, value_delimiter = ',')]
        algorithms: Option<Vec<SieveAlgorithm>>,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter)]
pub enum PrimeAlgorithm {
    Aks,
    MillerRabin,
    TrialDivision,
    TrialDivisionNewton,
    TrialDivisionSqrt,
}

impl PrimeAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrimeAlgorithm::Aks => "aks",
            PrimeAlgorithm::MillerRabin => "miller-rabin",
            PrimeAlgorithm::TrialDivision => "trial-division",
            PrimeAlgorithm::TrialDivisionNewton => "trial-division-newton",
            PrimeAlgorithm::TrialDivisionSqrt => "trial-division-sqrt",
        }
    }

    pub fn as_algorithm_fn(&self) -> fn(u64) -> bool {
        match self {
            PrimeAlgorithm::Aks => aks::is_prime,
            PrimeAlgorithm::MillerRabin => miller_rabin::is_prime,
            PrimeAlgorithm::TrialDivision => trial_division::is_prime,
            PrimeAlgorithm::TrialDivisionNewton => trial_division_newton::is_prime,
            PrimeAlgorithm::TrialDivisionSqrt => trial_division_sqrt::is_prime,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, EnumIter)]
pub enum SieveAlgorithm {
    SieveOfEratosthenes,
}

impl SieveAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            SieveAlgorithm::SieveOfEratosthenes => "sieve-of-eratosthenes",
        }
    }

    pub fn as_algorithm_fn(&self) -> fn(u64) -> Vec<u64> {
        match self {
            SieveAlgorithm::SieveOfEratosthenes => sieve_of_eratosthenes::sieve,
        }
    }
}
