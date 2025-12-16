/// Implementation using the derive pattern of clap.
use clap::Parser;

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

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
