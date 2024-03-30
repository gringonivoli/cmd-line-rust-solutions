use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of echo
struct Echor {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'))]
    omit_newline: bool,
}

impl Echor {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }

    pub fn newline(&self) -> &str {
        if self.omit_newline {
            ""
        } else {
            "\n"
        }
    }
}

fn main() {
    let echor = Echor::parse();
    print!("{}{}", echor.text(), echor.newline());
}
