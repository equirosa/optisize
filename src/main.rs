mod handle_mime;

use clap::Parser;
use mime::Mime;
use std::{path::Path, process};

use crate::handle_mime::get_mimetype;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
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
    let mimetype: Mime = get_mimetype(path);
    println!("Hello {}!", mimetype)
}
