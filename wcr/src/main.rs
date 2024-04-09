use wcr::Cli;

fn main() {
    if let Err(e) = wcr::run(Cli::default()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
