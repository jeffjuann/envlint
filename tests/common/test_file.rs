use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct TestFile 
{
  pub path: PathBuf
}

impl TestFile 
{
  pub fn new(test_dir: &TempDir, name: &str, contents: &str) -> Self 
  {
    let file_path = test_dir.path().join(name);
    let mut file = File::create(&file_path).expect("create testfile");
    file.write_all(contents.as_bytes()).expect("write to file");

    Self { path: file_path }
  }
}