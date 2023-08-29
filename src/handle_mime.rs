use mime::Mime;

pub(crate) fn get_mimetype(file: &str) -> Result<Mime, &'static str> {
    let parts: Vec<&str> = file.split('.').collect();
    match parts.last() {
        Some(v) => match *v {
            "png" => Ok(mime::IMAGE_PNG),
            "jpg" | "jpeg" => Ok(mime::IMAGE_JPEG),
            &_ => Err("Unknown type"),
        },
        None => Err("Extension unknown, I can't operate on that file"),
    }
}
