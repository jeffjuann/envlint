use std::{env::current_dir, io, path::PathBuf};

use crate::file::{check_file_in_directory, read_file};

use crate::parse::parse_variables;

use super::validate::validate_variable;

use crate::{emit_error, emit_info, emit_success, emit_warn};

use crate::cli::LintFlags;

use crate::common::{FileLine, VariableCollection};

pub fn lint(lint_flags: &LintFlags) -> io::Result<()>
{
  let cwd = if let Ok(cwd) = current_dir() {
    cwd
  } else {
    emit_error!("Unable to access current directory");
  };

  let mut env_template_file_name = if let Some(template) = &lint_flags.template {
    template
  } else {
    ".env.template"
  };

  let env_file_name = if let Some(file) = &lint_flags.file {
    file
  } else {
    ".env"
  };

  let mut env_template_file_path = cwd.join(env_template_file_name);
  let env_file_path = cwd.join(env_file_name);

  // check if the template file exists in the current directory
  if !check_file_in_directory(&env_file_path)
  {
    emit_error!("env file not found in the current directory");
  }

  // check if the template file exists in the current directory
  if !check_file_in_directory(&env_template_file_path)
  {
    // check if the template file using format .env.example exists in the current directory
    env_template_file_name = ".env.example";
    env_template_file_path = PathBuf::from(".env.example");
    if !check_file_in_directory(&env_template_file_path)
    {
      emit_warn!("Template file not found in the current directory");
      env_template_file_path = env_file_path.clone();
    }
  }

  
  emit_info!("checking environment variables template in file '{}'", &env_template_file_name);

  let file_content: Vec<FileLine> = read_file(&env_template_file_path)?;
  
  let template_environments: VariableCollection = parse_variables(&file_content, true)?;  

  emit_info!("checking environment variables in file '{}'", &env_file_name);

  let file_content: Vec<FileLine> = read_file(&env_file_path)?;

  let environments: VariableCollection = parse_variables(&file_content, false)?;  

  let env_filename = env_file_path.file_name().unwrap().to_str().unwrap().to_string();

  validate_variable(&env_filename, &environments, &template_environments);
 
  emit_success!("All environment variables in file '{}' are valid", env_filename);
  return Ok(());
}