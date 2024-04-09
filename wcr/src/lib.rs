use anyhow::Result;
use clap::Parser;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of wc
pub struct Cli {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}
impl Default for Cli {
    fn default() -> Self {
        let mut cli = Cli::parse();
        if [cli.words, cli.bytes, cli.chars, cli.lines]
            .iter()
            .all(|value| !value)
        {
            cli.lines = true;
            cli.words = true;
            cli.bytes = true;
        }
        cli
    }
}
impl Cli {
    pub fn files(&self) -> &Vec<String> {
        &self.files
    }

    pub fn lines(&self) -> bool {
        self.lines
    }
    pub fn words(&self) -> bool {
        self.words
    }
    pub fn bytes(&self) -> bool {
        self.bytes
    }
    pub fn chars(&self) -> bool {
        self.chars
    }
}

pub fn run(cli: Cli) -> Result<()> {
    let mut total_files_info = TotalFilesInfo::new(&cli);
    for filename in cli.files() {
        match ReadBuffer::of(filename) {
            Ok(read_buffer) => {
                let mut file_info = FileInfo::new(read_buffer);
                total_files_info.add_info(&mut file_info)?;
                println!(
                    "{}",
                    FmtFileInfo::new(&mut file_info, filename, &cli).as_string()?
                );
            }
            Err(error) => eprintln!("{filename}: {error}"),
        }
    }
    total_files_info.print();
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

struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
    read_buffer: ReadBuffer,
}
impl FileInfo {
    pub fn new(buffer: ReadBuffer) -> Self {
        FileInfo {
            num_lines: 0,
            num_words: 0,
            num_bytes: 0,
            num_chars: 0,
            read_buffer: buffer,
        }
    }

    pub fn num_words(&mut self) -> Result<usize> {
        self.count()?;
        Ok(self.num_words)
    }

    pub fn num_chars(&mut self) -> Result<usize> {
        self.count()?;
        Ok(self.num_chars)
    }

    pub fn num_bytes(&mut self) -> Result<usize> {
        self.count()?;
        Ok(self.num_bytes)
    }

    pub fn num_lines(&mut self) -> Result<usize> {
        self.count()?;
        Ok(self.num_lines)
    }

    fn count(&mut self) -> Result<()> {
        let mut raw_line = String::new();
        loop {
            let line_bytes = self.read_buffer.read_line(&mut raw_line)?;
            if line_bytes == 0 {
                break;
            }
            self.num_lines += 1;
            self.num_bytes += line_bytes;
            self.num_words += raw_line.split_whitespace().count();
            self.num_chars += raw_line.chars().count();
            raw_line.clear();
        }
        Ok(())
    }
}

struct FmtValue {
    a_value: usize,
    show: bool,
}
impl FmtValue {
    pub fn new(a_value: usize, show: bool) -> Self {
        Self { a_value, show }
    }
}
impl Display for FmtValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_display = String::from("");
        if self.show {
            to_display = format!("{:>8}", self.a_value);
        }
        write!(f, "{}", to_display)
    }
}

struct FmtFileInfo<'a> {
    a_file_info: &'a mut FileInfo,
    a_filename: String,
    a_cli: &'a Cli,
}
impl<'a> FmtFileInfo<'a> {
    pub fn new(a_file_info: &'a mut FileInfo, a_filename: &'a str, a_cli: &'a Cli) -> Self {
        Self {
            a_file_info,
            a_filename: a_filename.to_string(),
            a_cli,
        }
    }

    pub fn as_string(&mut self) -> Result<String> {
        Ok(format!(
            "{}{}{}{}{}",
            FmtValue::new(self.a_file_info.num_lines()?, self.a_cli.lines()),
            FmtValue::new(self.a_file_info.num_words()?, self.a_cli.words()),
            FmtValue::new(self.a_file_info.num_bytes()?, self.a_cli.bytes()),
            FmtValue::new(self.a_file_info.num_chars()?, self.a_cli.chars()),
            self.filename(),
        ))
    }

    fn filename(&self) -> String {
        if self.a_filename == "-" {
            "".to_string()
        } else {
            format!(" {}", self.a_filename)
        }
    }
}

struct TotalFilesInfo<'a> {
    cli: &'a Cli,
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}
impl<'a> TotalFilesInfo<'a> {
    pub fn new(cli: &'a Cli) -> Self {
        TotalFilesInfo {
            cli,
            num_lines: 0,
            num_words: 0,
            num_bytes: 0,
            num_chars: 0,
        }
    }

    pub fn as_string(&self) -> String {
        let lines = FmtValue::new(self.num_lines, self.cli.lines());
        let words = FmtValue::new(self.num_words, self.cli.words());
        let bytes = FmtValue::new(self.num_bytes, self.cli.bytes());
        let chars = FmtValue::new(self.num_chars, self.cli.chars());
        format!("{lines}{words}{bytes}{chars} total",)
    }

    pub fn add_info(&mut self, file_info: &mut FileInfo) -> Result<()> {
        self.num_lines += file_info.num_lines()?;
        self.num_words += file_info.num_words()?;
        self.num_bytes += file_info.num_bytes()?;
        self.num_chars += file_info.num_chars()?;
        Ok(())
    }

    pub fn print(&self) {
        if self.cli.files().len() > 1 {
            println!("{}", self.as_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const A_TEST_LINE: &str = "a test line\r\n";
    fn a_cli() -> Cli {
        Cli {
            files: vec!["".to_string(), "".to_string()],
            lines: true,
            words: true,
            bytes: true,
            chars: true,
        }
    }

    fn a_read_buffer() -> ReadBuffer {
        ReadBuffer {
            raw_buffer: Box::new(Cursor::new(A_TEST_LINE.to_string())),
        }
    }

    #[test]
    fn file_info_words() -> Result<()> {
        let mut file_info = FileInfo::new(a_read_buffer());

        assert_eq!(file_info.num_words()?, 3);
        Ok(())
    }

    #[test]
    fn file_info_chars() -> Result<()> {
        let mut file_info = FileInfo::new(a_read_buffer());

        assert_eq!(file_info.num_chars()?, 13);
        Ok(())
    }

    #[test]
    fn file_info_bytes() -> Result<()> {
        let mut file_info = FileInfo::new(a_read_buffer());

        assert_eq!(file_info.num_bytes()?, 13);
        Ok(())
    }

    #[test]
    fn file_info_lines() -> Result<()> {
        let mut file_info = FileInfo::new(a_read_buffer());

        assert_eq!(file_info.num_lines()?, 1);
        Ok(())
    }

    #[test]
    fn fmt_file_info() -> Result<()> {
        let cli = a_cli();
        let a_filename = "a_file_name";
        let mut file_info = FileInfo::new(a_read_buffer());
        let mut fmt_file_info = FmtFileInfo::new(&mut file_info, a_filename, &cli);

        assert_eq!(
            fmt_file_info.as_string()?,
            format!("       1       3      13      13 {}", a_filename)
        );
        Ok(())
    }

    #[test]
    fn fmt_value() {
        let fmt_value = FmtValue::new(5, true);

        assert_eq!(format!("{fmt_value}"), "       5");
    }

    #[test]
    fn total_files_info() -> Result<()> {
        let cli = a_cli();
        let mut file_info = FileInfo::new(a_read_buffer());
        let mut total_files_info = TotalFilesInfo::new(&cli);

        total_files_info.add_info(&mut file_info)?;

        assert_eq!(
            format!("{}", total_files_info.as_string()),
            "       1       3      13      13 total"
        );
        Ok(())
    }
}
