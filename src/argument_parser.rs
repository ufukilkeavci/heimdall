use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    include_capitals: bool,

    #[arg(short, long)]
    include_letters: bool,

    #[arg(short, long)]
    includes_numbers: bool,

    #[arg(short, long)]
    include_symbols: bool,
}
