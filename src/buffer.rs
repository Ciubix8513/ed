use std::path::PathBuf;

///Main buffer that is being edited
pub struct Buffer {
    ///The actual text of the file
    pub buffer: String,
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Self {
        Buffer {
            buffer: if let Some(path) = path {
                if !path.exists() {
                    println!("{}: No such file or directory", path.display());
                }
                if path.is_dir() {
                    println!("{}: Is a directory", path.display());
                }
                let file = std::fs::read(path).unwrap();
                println!("{}", file.len());
                String::from_utf8(file).unwrap()
            } else {
                String::new()
            },
        }
    }
}
