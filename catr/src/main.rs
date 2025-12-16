// Implementation using the derive pattern of clap.
use anyhow::Result;
use clap::Parser;

use std::io::{self, BufRead, BufReader};

mod filesys;
use filesys::{open, printfile};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust ver. of `cat`
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
    files: Vec<String>,

    /// Number the output lines, starting at 1
    #[arg(short('n'), long("number"))]
    number_lines: bool,

    /// Number the non-blank output lines, starting at 1
    #[arg(short('b'), long("number-nonblank"), conflicts_with("number_lines"))]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open file {}: {}", filename, e),
            Ok(buf) => printfile(buf, args.number_lines, args.number_nonblank_lines)?,
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
