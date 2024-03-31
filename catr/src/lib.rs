use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of cat
pub struct Cli {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}
impl Cli {
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }

    pub fn number_lines(&self) -> bool {
        self.number_lines
    }

    pub fn number_nonblank_lines(&self) -> bool {
        self.number_nonblank_lines
    }
}

pub fn run(cli: &Cli) -> Result<()> {
    for filename in cli.files() {
        match open(filename) {
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let raw_line = line_result?;
                    let mut line: Box<dyn Printeable> = Box::new(Line::new(&raw_line));
                    if cli.number_lines() {
                        line = Box::new(NumberedLine::new(&raw_line, line_num));
                    } else if cli.number_nonblank_lines() {
                        let a_line =
                            NumberedNonblankLine::new(NumberedLine::new(&raw_line, prev_num));
                        prev_num += a_line.counted();
                        line = Box::new(a_line);
                    }
                    line.print();
                }
            }
            Err(e) => eprintln!("{filename}: {e}"),
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

trait Printeable {
    fn print(&self);
}

struct Line {
    a_line: String,
}
impl Line {
    fn new(a_line: &str) -> Self {
        Line {
            a_line: a_line.to_string(),
        }
    }
}
impl Printeable for Line {
    fn print(&self) {
        println!("{}", self.a_line);
    }
}

struct NumberedLine {
    a_line_number: usize,
    a_line: String,
}
impl NumberedLine {
    pub fn new(a_line: &str, a_line_number: usize) -> Self {
        NumberedLine {
            a_line_number,
            a_line: a_line.to_string(),
        }
    }

    pub fn line(&self) -> &str {
        &self.a_line
    }
}
impl Printeable for NumberedLine {
    fn print(&self) {
        println!("{:6}\t{}", self.a_line_number + 1, self.a_line);
    }
}

struct NumberedNonblankLine {
    a_numbered_line: NumberedLine,
}
impl NumberedNonblankLine {
    fn new(a_numbered_line: NumberedLine) -> Self {
        Self { a_numbered_line }
    }

    fn counted(&self) -> usize {
        if self.a_numbered_line.line().is_empty() {
            0
        } else {
            1
        }
    }
}
impl Printeable for NumberedNonblankLine {
    fn print(&self) {
        if self.a_numbered_line.line().is_empty() {
            println!();
        } else {
            self.a_numbered_line.print();
        }
    }
}
