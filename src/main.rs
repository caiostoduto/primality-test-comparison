mod algorithms;
mod cli;

use clap::Parser;
use cli::parsing::*;

fn main() {
    let cli_parsed = Cli::parse();

    match &cli_parsed.command {
        Commands::Benchmark {
            duration,
            algorithm,
            output_path,
        } => {
            cli::benchmark::handle_cli(duration, algorithm, output_path);
        }
        Commands::Test { number, algorithm } => {
            cli::test::handle_cli(*number, algorithm);
        }
    }
}
