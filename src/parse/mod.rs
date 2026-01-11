mod matcher;

mod builder;

use std::{collections::HashMap, io, str::FromStr};

use crate::emit_error;

use crate::common::{FileLine, LineType, VariableCollection, VariableMetadata, VariableTag, VariableTagErrorKind};

pub fn parse_variables(env_file_content: &Vec<FileLine>, is_template_file: bool) -> io::Result<VariableCollection>
{
  let mut environment_variables: VariableCollection = VariableCollection::new();
  let mut current_env: HashMap<String, (String, u16)> = HashMap::new();

  let mut line_index: u16 = 1;
  for line in env_file_content
  {
    line_index = line.index;
    let line = line.line.as_str().trim();
    match check_line(&line)
    {
      LineType::Empty | LineType::Comment =>
      {
        if !current_env.is_empty()
        {  
          current_env.clear();
        }
      },
      LineType::Tag if is_template_file =>
      {
        // Only parse tag in template file
        let tag: VariableMetadata = parse_tag_line(&line, line_index);
        current_env.insert(tag.name.to_string(), (tag.value, line_index));
      },
      LineType::Env => 
      {  
        let (env_key, env_value) = parse_env_line(&line, line_index);
        current_env.insert(String::from("key"), (env_key, line_index));
        current_env.insert(String::from("value"), (env_value, line_index));
      
        let env = builder::build_env(&current_env, line_index);   
        environment_variables.add(env, line_index);
        current_env.clear();
      },
      LineType::Invalid =>
      {
        emit_error!("Invalid syntax at line: {}", line_index);
      }
      _ => {},
    }
  }

  // process the last record, if current_record is not empty
  if !current_env.is_empty()
  {
    let env = builder::build_env(&current_env, line_index-1);
    environment_variables.add(env, line_index-1);
  }

  return Ok(environment_variables);
}

pub fn check_line(line: &str) -> LineType
{
  if line.is_empty()
  {
    return LineType::Empty;
  }
  else if matcher::TAG_MATCHER.is_match(line)
  {
    return LineType::Tag;
  }
  else if matcher::ENV_MATCHER.is_match(line)
  {
    return LineType::Env;
  }
  else if matcher::COMMENT_MATCHER.is_match(line)
  {
    return LineType::Comment;
  }
  return LineType::Invalid
}

pub fn parse_tag_line(line: &str, line_index: u16) -> VariableMetadata
{
  if let Some(captures) = matcher::TAG_MATCHER.captures(&line)
  {
    let tag_name = match VariableTag::from_str(captures.get(1).map_or("", |m| m.as_str()))
    {
      Ok(name) => name,
      Err(VariableTagErrorKind::Unknown(tag_str)) => 
      {
        emit_error!("Invalid tag '{}' at line {}", tag_str, line_index);
      },
    };

    let value: &str = match captures.get(2).map(|m| m.as_str().trim())
    {
      Some(value) if !value.is_empty() => value,
      _ => captures.get(3).map_or("", |m| m.as_str()),
    };

    return VariableMetadata { name: tag_name, value: value.to_string() };
  } 
  else 
  {
    emit_error!("Invalid syntax at line: {}", line_index);
  }
}

pub fn parse_env_line(line: &str, line_index: u16) -> (String, String)
{
  if let Some(captured) = matcher::ENV_MATCHER.captures(&line)
  { 
    let env_key = match captured.get(1).map(|m| m.as_str())
    {
      Some(key) if !key.is_empty() => key,
      _ =>
      {
        emit_error!("Invalid syntax at line: {}", line_index);
      }
    };
    
    let env_value = match captured.get(2).map(|m| m.as_str()) {
      Some(value) if !value.trim().is_empty() => value,
      _ => captured.get(3).map_or("", |m| m.as_str()),
    };

    return (env_key.to_string(), env_value.to_string());
  } 
  else 
  {
    emit_error!("Invalid syntax at line: {}", line_index);
  }
}