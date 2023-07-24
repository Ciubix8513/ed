use std::path::PathBuf;

///Main buffer that is being edited
pub struct Buffer {
    ///The actual text of the file, stored as an array of string for easier modification
    pub lines: Vec<String>,
    pub cursor: usize,
    pub modified: bool,
}
pub fn string_to_lines(input: &str) -> Vec<String> {
    input
        .trim_end_matches('\n')
        .split('\n')
        .map(|i| Into::<String>::into(i))
        .collect()
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            lines: path.map_or(Vec::new(), |path| {
                if !path.exists() {
                    println!("{}: No such file or directory", path.display());
                }
                if path.is_dir() {
                    println!("{}: Is a directory", path.display());
                }
                let file = std::fs::read(path).unwrap();
                println!("{}", file.len());
                string_to_lines(&String::from_utf8(file).unwrap())
            }),
            cursor: 0,
            modified: false,
        }
    }
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}
