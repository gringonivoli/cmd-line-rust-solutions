use clap::Parser;
use headr::Cli;

fn main() {
    if let Err(e) = headr::run(&Cli::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
