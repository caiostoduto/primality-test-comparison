mod algorithms;
mod cli;

use clap::Parser;
use cli::parsing::*;

fn main() {
    let cli_parsed = Cli::parse();

    match &cli_parsed.command {
        Commands::Benchmark {
            duration,
            algorithms,
            output_path,
            save,
        } => {
            cli::benchmark::handle_cli(duration, algorithms, output_path, save);
        }
        Commands::Test { number, algorithms } => {
            cli::test::handle_cli(*number, algorithms);
        }
    }
}
