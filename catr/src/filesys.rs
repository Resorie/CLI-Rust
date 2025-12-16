use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;

pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn printfile(
    buffer: Box<dyn BufRead>,
    number_lines: bool,
    number_nonblank_lines: bool,
) -> Result<()> {
    let mut line_number = 1;

    for line in buffer.lines() {
        let line = line?;

        if number_lines || (!line.is_empty() && number_nonblank_lines) {
            print!("{:6}\t", line_number);
            line_number += 1;
        }

        println!("{}", line);
    }

    Ok(())
}
