use clap::Parser;
use std::{
    io,
    path::{Path, PathBuf},
    process::{self, exit, Command},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Convert <INPUT> to a more space-efficient file format. Usually lossy."
    )]
    convert: bool,
    #[arg(help = "File to be optimized")]
    input: String,
}

fn main() {
    let args: Args = Args::parse();
    let path: &Path = Path::new(&args.input);
    if !path.exists() || !path.is_file() {
        eprintln!("Input must be a valid path and a file");
        exit(1);
    }
    if let Some(extension) = path.extension() {
        handle_extension(extension, path)
    } else {
        eprintln!("File has no extension, and I can't decide what to do with it.");
        exit(1);
    }
}

fn handle_extension(extension: &std::ffi::OsStr, path: &Path) {
    if let Some(extension_str) = extension.to_str() {
        match extension_str.to_lowercase().as_str() {
            "jpeg" | "jpg" => optimize_jpeg(path),
            _ => {
                eprintln!("I don't know how to handle the {} extension", extension_str);
                exit(1);
            }
        }
    } else {
        eprintln!("Extension {:?} couldn't be parsed", extension);
        exit(1);
    }
}

fn optimize_jpeg(original_path: &Path) {
    let out_path_option: Option<PathBuf> = add_optimized_suffix(original_path);
    if let Some(out_path_str) = out_path_option {
        let cmd_output = Command::new("cjpeg")
            .arg("-optimize")
            .arg("-progressive")
            .arg("-outfile")
            .arg(&out_path_str)
            .arg(original_path)
            .output();
        handle_cmd_output(cmd_output);
    } else {
        eprintln!("Couldn't determine new file path.");
        exit(1);
    }
}

fn add_optimized_suffix(original_path: &Path) -> Option<PathBuf> {
    let file_name: Option<&str> = original_path.file_name().and_then(|string| string.to_str());
    if let Some(name) = file_name {
        let mut new_name: String = String::from(name);
        if let Some(dot_idx) = new_name.rfind('.') {
            new_name.insert_str(dot_idx, "-optimized");
        } else {
            new_name.push_str("-optimized");
        }

        let mut new_path = original_path.to_path_buf();
        new_path.set_file_name(new_name);
        Some(new_path)
    } else {
        None
    }
}

fn handle_cmd_output(cmd_output: Result<process::Output, io::Error>) {
    match cmd_output {
        Ok(o) => {
            if !o.status.success() {
                if let Some(code) = o.status.code() {
                    eprintln!("Command failed with error code: {}", code);
                    exit(code);
                }
                eprintln!("Failed with unknown error code");
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
