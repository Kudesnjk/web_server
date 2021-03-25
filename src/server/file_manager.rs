use std::fs;
use std::path;
use std::io;

pub const DEFAULT_PATH: &'static str = "index.html";

pub fn new_root_dir(document_root: &str) -> Option<path::PathBuf> {
    let path = path::Path::new(document_root);
    if !path.exists() || !path.is_dir() {
        return None
    }

    Some(path.to_path_buf())
}

pub fn get_file(root_path: &path::Path, filename: &mut String) -> io::Result<fs::File> {
    let mut file_abs_path = root_path.join(
        path::Path::new(&filename[1..filename.len()]));

    if file_abs_path.is_dir() {
        file_abs_path = file_abs_path.join(DEFAULT_PATH);
        filename.push_str(DEFAULT_PATH);
    }

    let file = fs::File::open(file_abs_path)?;
    Ok(file)
}

pub fn get_mime_type(filename: &String) -> Option<&str> {
    let path = path::Path::new(filename);
    let extention = path.extension()?.to_str()?;
    let mime_type = match extention {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "swf" => "application/x-shockwave-flash",
        _ => "text/plain",
    };
    Some(mime_type)
}