use std::fs;

use anyhow::Result;

pub fn read_file(filename: &str) -> Result<String> {
    let mut path = std::path::PathBuf::new();
    path.push("data");
    path.push(filename);
    Ok(fs::read_to_string(path)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
