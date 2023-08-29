mod handle_mime;

use clap::Parser;
use mime::Mime;
use std::{path::Path, process};

use crate::handle_mime::get_mimetype;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.input);
    if !path.exists() || !path.is_file() {
        eprintln!("Input must be a valid path and a file");
        process::exit(1);
    }
    let mimetype: Result<Mime, &str> = get_mimetype(&args.input);
    match mimetype {
        Ok(mtype) => println!("Hello {}!", mtype),
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1)
        }
    }
}
