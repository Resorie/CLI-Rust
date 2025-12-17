use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;

pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn print_bytes(mut buffer: Box<dyn BufRead>, bytes: u64) -> Result<()> {
    let data = buffer.fill_buf()?;
    let data = &data[..std::cmp::min(bytes as usize, data.len())];

    print!("{}", String::from_utf8_lossy(&data));
    Ok(())
}

pub fn print_lines(buffer: Box<dyn BufRead>, lines: u64) -> Result<()> {
    for (line_count, line) in buffer.split(b'\n').enumerate() {
        if (line_count as u64) >= lines {
            break;
        }

        let line = line?;
        println!("{}", String::from_utf8(line)?);
    }
    Ok(())
}
