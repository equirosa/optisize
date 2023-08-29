use std::path::Path;

use mime::Mime;

pub(crate) fn get_mimetype(path: &Path) -> Result<Mime, &'static str> {
    return if let Some(path_str) = path.to_str() {
        let path_string: String = path_str.to_string();
        let parts: Vec<&str> = path_string.split('.').collect();
        match parts.last() {
            Some(v) => match *v {
                "png" => Ok(mime::IMAGE_PNG),
                "jpg" | "jpeg" => Ok(mime::IMAGE_JPEG),
                &_ => Err("Unknown type"),
            },
            None => Err("Extension unknown, I can't operate on that file"),
        }
    } else {
        Err("Couldn't parse string")
    };
}
