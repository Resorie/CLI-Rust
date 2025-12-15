use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
// Rust ver. of `echo`
struct Args {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Omit newline at line end
    #[arg(short('n'))]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
