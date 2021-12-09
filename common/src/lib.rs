use std::fs;

pub fn read_file(filename: &str) -> String {
    let mut path = std::path::PathBuf::new();
    path.push("data");
    path.push(filename);
    fs::read_to_string(path).unwrap()
}
