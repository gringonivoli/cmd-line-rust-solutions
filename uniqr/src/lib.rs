use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of uniq
pub struct Cli {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}
impl Cli {
    // TODO:
}

pub fn run(cli: Cli) -> Result<()> {
    // TODO:
    // match ReadBuffer::of()
    println!("{:?}", cli);
    Ok(())
}

struct ReadBuffer {
    raw_buffer: Box<dyn BufRead>,
}
impl ReadBuffer {
    pub fn of(a_filename: &str) -> Result<ReadBuffer> {
        Ok(ReadBuffer {
            raw_buffer: match a_filename {
                "-" => Box::new(BufReader::new(io::stdin())),
                _ => Box::new(BufReader::new(File::open(a_filename)?)),
            },
        })
    }

    pub fn read_line(&mut self, a_string_to_write: &mut String) -> Result<usize> {
        Ok(self.raw_buffer.read_line(a_string_to_write)?)
    }
}
