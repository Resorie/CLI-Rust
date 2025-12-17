use anyhow::Result;
use clap::Parser;

mod filesys;
use filesys::{open, print_bytes, print_lines};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust ver. of `head`
struct Args {
    /// Input file(s)
    #[arg(default_value("-"), value_name("FILE"))]
    files: Vec<String>,

    /// Print _COUNT_ lines of the files
    #[arg(
        default_value("10"),
        value_name("LINES"),
        short('n'),
        long,
        value_parser(clap::value_parser!(u64).range(1..))
    )]
    lines: u64,

    /// Print _BYTES_ of the files
    #[arg(
        value_name("BYTES"),
        short('c'),
        long,
        conflicts_with("lines"),
        value_parser(clap::value_parser!(u64).range(1..))
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut file_num = 1;

    for filename in &args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open file {}: {}", filename, e),
            Ok(buf) => {
                if file_num >= 2 {
                    print!("\n");
                }
                if args.files.len() >= 2 {
                    println!("==> {} <==", filename);
                }

                if let Some(bytes) = args.bytes {
                    print_bytes(buf, bytes)?;
                } else {
                    print_lines(buf, args.lines)?;
                }
            }
        }

        file_num += 1;
    }
    Ok(())
}
