use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of head
pub struct Cli {
    /// Input files(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(value_name = "LINES", short('n'), long, default_value = "10", value_parser = clap::value_parser!(u64).range(1..))]
    lines: u64,

    /// Number of bytes
    #[arg(value_name = "BYTES", short('c'), long, conflicts_with("lines"), value_parser = clap::value_parser!(u64).range(1..))]
    bytes: Option<u64>,
}
impl Cli {
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }

    pub fn lines(&self) -> u64 {
        self.lines
    }

    pub fn bytes(&self) -> Option<u64> {
        self.bytes
    }
}

pub fn run(cli: &Cli) -> Result<()> {
    let num_files = cli.files().len();
    for (file_num, filename) in cli.files().iter().enumerate() {
        match ReadBuffer::of(filename) {
            Ok(read_buffer) => {
                FileHeader::new(num_files, file_num, filename).print();
                if let Some(num_bytes) = cli.bytes() {
                    BytesOf::new(read_buffer, num_bytes as usize).print()?;
                } else {
                    LinesOf::new(read_buffer, cli.lines()).print()?;
                }
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }
    Ok(())
}

struct LinesOf {
    read_buffer: ReadBuffer,
    lines: u64,
}
impl LinesOf {
    fn new(read_buffer: ReadBuffer, lines: u64) -> Self {
        Self { read_buffer, lines }
    }

    fn print(&mut self) -> Result<()> {
        let mut line = String::new();
        for _ in 0..self.lines {
            let bytes_read = self.read_buffer.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            print!("{line}");
            line.clear();
        }
        Ok(())
    }
}

struct BytesOf {
    read_buffer: ReadBuffer,
    num_bytes: usize,
}
impl BytesOf {
    fn new(read_buffer: ReadBuffer, num_bytes: usize) -> Self {
        Self {
            num_bytes,
            read_buffer,
        }
    }

    fn print(&mut self) -> Result<()> {
        let mut buffer = vec![0; self.num_bytes];
        let bytes_read = self.read_buffer.read(&mut buffer)?;
        print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
        Ok(())
    }
}

struct FileHeader {
    num_files: usize,
    file_num: usize,
    filename: String,
}

impl FileHeader {
    fn new(num_files: usize, file_num: usize, filename: &str) -> Self {
        Self {
            num_files,
            file_num,
            filename: filename.to_string(),
        }
    }

    fn print(&self) {
        if self.num_files > 1 {
            println!(
                "{}==> {} <==",
                if self.file_num > 0 { "\n" } else { "" },
                self.filename
            );
        }
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

    pub fn read_line(&mut self, a_string_to_write: &mut String) -> Result<usize> {
        Ok(self.raw_buffer.read_line(a_string_to_write)?)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        Ok(self.raw_buffer.read(buffer)?)
    }
}
