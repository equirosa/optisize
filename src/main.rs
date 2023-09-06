use clap::Parser;
use std::{path::Path, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Have optisize convert <INPUT> to a new, more space-efficient file format. (NOTE: Usually lossy)"
    )]
    convert: bool,
    #[arg(help = "File to be optimized")]
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
        handle_extension(extension, path)
    } else {
        eprintln!("File has no extension, and I can't decide what to do with it.");
        process::exit(1);
    }
}

fn handle_extension(extension: &std::ffi::OsStr, path: &Path) {
    if let Some(extension_str) = extension.to_str() {
        match extension_str.to_lowercase().as_str() {
            "jpeg" | "jpg" => optimize_jpeg(path),
            "png" => optimize_png(path),
            "mp4" => optimize_video(path),
            _ => {
                eprintln!("I don't know how to handle the {} extension", extension_str);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Extension {:?} couldn't be parsed", extension);
        process::exit(1);
    }
}

fn optimize_video(path: &Path) {
    println!("{:?}", path)
}

fn optimize_png(path: &Path) {
    println!("{:?}", path)
}

fn optimize_jpeg(path: &Path) {
    println!("{:?}", path)
}
