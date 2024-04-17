use clap::Parser;
use uniqr::Cli;

fn main() {
    if let Err(e) = uniqr::run(Cli::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
