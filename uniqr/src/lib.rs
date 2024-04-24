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
    pub fn in_file(&self) -> &str {
        &self.in_file
    }
    pub fn out_file(&self) -> Option<&str> {
        self.out_file.as_deref()
    }
    pub fn count(&self) -> bool {
        self.count
    }
}

pub fn run(cli: Cli) -> Result<()> {
    match ReadBuffer::of(cli.in_file()) {
        Ok(mut read_buffer) => {
            let mut line = String::new();
            loop {
                let bytes = read_buffer.read_line(&mut line)?;
                if bytes.is_zero() {
                    break;
                }
            }
            print!("{}", line);
            line.clear();
        }
        Err(error) => eprintln!("{}: {}", cli.in_file(), error),
    }
    Ok(())
}

struct ReadBytes {
    raw_bytes: usize,
}
impl ReadBytes {
    pub fn is_zero(&self) -> bool {
        self.raw_bytes == 0
    }
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

    pub fn read_line(&mut self, a_string_to_write: &mut String) -> Result<ReadBytes> {
        Ok(ReadBytes {
            raw_bytes: self.raw_buffer.read_line(a_string_to_write)?,
        })
    }
}
