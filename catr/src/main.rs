use catr::Cli;
use clap::Parser;

fn main() {
    if let Err(e) = catr::run(&Cli::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
