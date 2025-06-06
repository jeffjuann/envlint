use assert_cmd::Command;
use tempfile::{tempdir, TempDir};

use crate::common::test_file::TestFile;

use std::str::from_utf8;

pub struct TestEnvironment 
{
  dir: TempDir,
  outputs: Vec<String>,
  status: bool,
}

#[allow(dead_code)]
impl TestEnvironment 
{
  pub fn new() -> Self 
  {
    let temp_dir = match tempdir() 
    {
      Ok(dir) => dir,
      Err(e) => panic!("Unable to create temporary test directory: {}", e),
    };

    Self {
      dir: temp_dir,
      outputs: Vec::new(),
      status: false,
    }
  }

  pub fn print_outputs(&self)
  {
    println!("Outputs:");
    for output in self.outputs.iter() 
    {
      println!("{}", output);
    }
  }

  pub fn is_success(&self) -> bool
  {
    self.status
  }

  pub fn is_failed(&self) -> bool
  {
    !self.status
  }

  pub fn output_contains(&self, value: &str) -> bool
  {
    let contains = self.outputs.iter().any(|output| output.contains(value));

    if contains
    {
      println!("Output: {:?}", self.outputs.join("\n"));
    }

    return contains;
  }

  /// Explicitly panic if unable to remove TestEnvironment from filesystem
  pub fn close(&mut self) 
  {
      let dir = std::mem::replace(&mut self.dir, tempdir().expect("create temporary test directory"));
      dir.close().expect("remove temporary test directory");
      self.outputs.iter().for_each(|output| println!("{}", output));
  }

  pub fn create_file(&self, name: &str, contents: &str) -> TestFile 
  {
    TestFile::new(&self.dir, name, contents)
  }
  /// Create a TestFile within the TestEnvironment
  pub fn create_env_file(&self, contents: &str) -> TestFile 
  {
    TestFile::new(&self.dir, ".env", contents)
  }

  pub fn create_template_file(&self, contents: &str) -> TestFile 
  {
    TestFile::new(&self.dir, ".env.template", contents)
  }

  fn init_cmd(&self) -> Command 
  {
    return Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command from binary name");
  }

  pub fn run (&mut self, args: &[&str])
  {
    let mut cmd = self.init_cmd();

    cmd.current_dir(&self.dir);
    cmd.args(args);

    let output = cmd.output().expect("command output");
    let stdout = from_utf8(&output.stdout).expect("stdout");
    self.outputs.push(stdout.to_string());

    self.status = self.outputs.iter().any(|output| output.contains("success"));

    self.close();
  }

  pub fn run_lint (&mut self, args: &[&str])
  {
    let mut cmd_args = vec!["lint"];
    cmd_args.extend_from_slice(args);
    self.run(cmd_args.as_slice());
  }

  // Test a lint command
  pub fn test_lint (&mut self, env_content: &str, template_env_content: &str, args: &[&str], expected_status: bool, expected_output: &str)
  {
    let template_file = self.create_template_file(template_env_content);
    let env_file = self.create_env_file(env_content);

    let mut cmd_args = vec!["-f", env_file.path.to_str().unwrap(), "-t", template_file.path.to_str().unwrap()];
    cmd_args.extend_from_slice(args);
    self.run_lint(&cmd_args);

    self.print_outputs();

    if expected_output.trim() == ""
    {
      assert_eq!(expected_status, self.is_success());
    }
    else {
      assert!(self.output_contains(expected_output));
    }
  }
}