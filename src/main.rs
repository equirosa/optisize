use clap::Parser;
use mime::Mime;
use std::{fs::File, path::Path, process};

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

fn get_mimetype(path: &Path) -> Mime {
    if let Some(path_str) = path.to_str() {
        let path_string: String = path_str.to_string();
        let parts: Vec<&str> = path_string.split('.').collect();
        return match parts.last() {
            None => mime::TEXT_PLAIN,
            Some(v) => match *v {
                "png" => mime::IMAGE_PNG,
                "jpg" | "jpeg" => mime::IMAGE_JPEG,
                &_ => mime::TEXT_PLAIN,
            },
        };
    } else {
        eprintln!("Couldn't parse string");
        process::exit(1);
    }
}
