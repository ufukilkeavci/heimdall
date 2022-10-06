mod password_generator;
mod server;

use std::net::TcpListener;

use clap::Parser;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use rouille::Request;
use rouille::Response;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value = "16")]
    length: u8,

    #[arg(long)]
    exclude_capitals: bool,

    #[arg(long)]
    exclude_letters: bool,

    #[arg(long)]
    exclude_numbers: bool,

    #[arg(long)]
    exclude_symbols: bool,

    #[arg(long, short)]
    copy: bool,

    #[arg(long, short)]
    port: Option<i32>,
}

fn main() {
    let args = Args::parse();

    if args.port.is_none() {
        let password = password_generator::generate(
            Some(args.length),
            Some(!args.exclude_numbers),
            Some(!args.exclude_letters),
            Some(!args.exclude_capitals),
            Some(!args.exclude_symbols),
            Option::Some(Box::new(vec!['\0'])),
        )
        .unwrap();

        if !args.copy {
            println!("{}", password);
        } else {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(*password.to_owned()).unwrap();
        }
    } else {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            server::handle_connection(stream);
        }
    }
}
