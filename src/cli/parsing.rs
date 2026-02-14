use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::algorithms::*;

#[derive(Parser)]
#[command(name = "Primality Test Comparison")]
#[command(version = "1.0.0")]
#[command(about = "Does awesome things", long_about = None)]
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

        /// Algorithm to benchmark (e.g., trial-division, miller-rabin, etc.)
        #[arg(value_enum)]
        algorithm: Option<Algorithm>,

        /// Sets a custom output folder for the results (default: ./out)
        #[arg(short, long, value_name = "FOLDER", default_value = "./out")]
        output_path: PathBuf,
    },
    Test {
        /// Number to test for primality
        number: u64,

        /// Algorithm to use for testing (e.g., trial-division, miller-rabin, etc.)
        #[arg(value_enum)]
        algorithm: Option<Algorithm>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Algorithm {
    TrialDivision,
    TrialDivisionSqrt,
}

impl Algorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            Algorithm::TrialDivision => "trial-division",
            Algorithm::TrialDivisionSqrt => "trial-division-sqrt",
        }
    }

    pub fn as_algorithm_fn(&self) -> fn(u64) -> bool {
        match self {
            Algorithm::TrialDivision => trial_division::is_prime,
            Algorithm::TrialDivisionSqrt => trial_division_sqrt::is_prime,
        }
    }
}
