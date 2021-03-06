use std::fs;

pub fn read_file(filename: &str) -> String {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("data");
    path.push(filename);
    fs::read_to_string(path).unwrap()
}

pub fn read_2d(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}
