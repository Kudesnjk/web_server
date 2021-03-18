use std::fs;
use std::path;
use std::io;

enum ContentTypes {
    HTML,
    CSS,
    JS,
    JPG,
    JPEG,
    PNG,
    GIF,
    SWF,
}

pub fn new_root_dir(document_root: &str) -> Option<path::PathBuf> {
    let path = path::Path::new(document_root);
    if !path.exists() || !path.is_dir() {
        return None
    }

    Some(path.to_path_buf())
}

pub fn get_file(root_path: &path::Path, filename: &str) -> io::Result<fs::File> {
    let file_abs_path = root_path.join(path::Path::new(filename));        
    let file = fs::File::open(file_abs_path)?;
    Ok(file)
}