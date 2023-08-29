use std::{path::Path, process};

use mime::Mime;

pub(crate) fn get_mimetype(path: &Path) -> Mime {
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
