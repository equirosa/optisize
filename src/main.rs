use clap::Parser;
use std::{path::Path, process};

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
    if let Some(extension) = path.extension() {
        handle_extension(extension)
    } else {
        eprintln!("File has no extension, and I can't decide what to do with it.");
        process::exit(1);
    }
}

fn handle_extension(extension: &std::ffi::OsStr) {
    println!("Extension is {:?}", extension);
}
