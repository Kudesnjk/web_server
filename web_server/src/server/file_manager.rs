use std::fs;
use std::path;
use std::io;
use std::io::Read;

pub fn new_root_dir(document_root: &str) -> Option<path::PathBuf> {
    let path = path::Path::new(document_root);
    if !path.exists() || !path.is_dir() {
        return None
    }

    Some(path.to_path_buf())
}

pub fn get_file(root_path: &path::Path, filename: &str) -> io::Result<fs::File> {
    let file_abs_path = root_path.join(path::Path::new(&filename[1..filename.len()]));
    let file = fs::File::open(file_abs_path)?;
    Ok(file)
}

pub fn get_mime_type(filename: &str) -> Option<&str> {
    let path = path::Path::new(filename);
    let extention = path.extension()?.to_str()?;
    match extention {
        "html" => Some("text/html"),
        "css" => Some("text/css"),
        "js" => Some("text/javascript"),
        "jpg" => Some("image/jpeg"),
        "jpeg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "gif" => Some("image/gif"),
        "swf" => Some("application/vnd"),
        _ => Some("text/plain"),
    }
}