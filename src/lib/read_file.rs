use std::fs;

pub fn get_file_content(path: String) -> String {
    fs::read_to_string(path).expect("Should have been able to read the file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{ErrorKind, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_file_content_existing_file() {
        let content = "Test content";
        let file = NamedTempFile::new().expect("Failed to create temporary file");
        let file_path = file.path().to_str().expect("Failed to get file path").to_string();
        {
            let mut file = file.reopen().expect("Failed to reopen the file");
            file.write_all(content.as_bytes()).expect("Failed to write to the file");
        }
        let result = get_file_content(file_path);
        assert_eq!(result, content);
    }
}
