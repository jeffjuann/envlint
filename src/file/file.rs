use std::{env, fs, io, path::PathBuf};

use crate::emit_error;

use crate::common::FileLine;

pub fn check_file_in_directory(file_name: &PathBuf) -> bool
{
  match env::current_dir()
  {
    Ok(current_dir) => 
    {
      let file_path = current_dir.join(file_name);

      if file_path.exists()
      {
        return true;
      }
      else
      {
        return false;
      }
    },
    Err(_) => 
    {
      emit_error!("Unable to access current directory");
    }
  };
}

pub fn read_file(file_path: &PathBuf) -> io::Result<Vec<FileLine>>
{
  let file_content = fs::read_to_string(file_path)?;
  let mut result: Vec<FileLine> = Vec::new();
  let mut current_line = String::new();
  let mut inside_quotes = false;

  let mut current_line_index: u16 = 1;
  let mut line_index: u16 = 1;
  for ch in file_content.chars() 
  {
    if ch == '"' 
    {
      if inside_quotes 
      {
        // only toggle inside quotes if the quote is not escaped and the backslash is not escaped
        if !current_line.ends_with('\\') || current_line.ends_with("\\\\")
        {
          inside_quotes = !inside_quotes;
        }
      }
      else
      {
        inside_quotes = !inside_quotes;
      }
    }
    
    if ch == '\n' && !inside_quotes 
    {
      // When a newline is encountered outside quotes, push current line
      result.push(FileLine{line: current_line, index: current_line_index});
      current_line_index = line_index;
      current_line = String::new();
    } 
    else 
    {
      // Otherwise, keep appending the character to the current line
      current_line.push(ch);
    }

    if ch == '\n'
    {
      line_index += 1;
      if !inside_quotes
      {
        current_line_index = line_index;
      }
    }
  }

  if !current_line.is_empty() 
  {
    result.push(FileLine{line: current_line, index: current_line_index});
  }
  
  return Ok(result);
}