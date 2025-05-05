use std::fs;
use std::io;
use std::path::Path;

pub fn create_dir_all(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dest_path = dest.join(entry.file_name());
        fs::copy(entry.path(), dest_path)?;
    }
    Ok(())
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_create_and_copy_dir_and_write_file() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("src");
        create_dir_all(&src).unwrap();
        let file_path = src.join("test.txt");
        write_file(&file_path, "hello").unwrap();
        let dest = dir.path().join("dest");
        copy_dir(&src, &dest).unwrap();
        let dest_file = dest.join("test.txt");
        let content = fs::read_to_string(dest_file).unwrap();
        assert_eq!(content, "hello");
    }
}