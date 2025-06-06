use std::{collections::HashMap, io, str::FromStr};

use regex::{Captures, Regex};

use crate::{emit_error, emit_warn};

use crate::common::{VariableMetadata, VariableTag, FileLine, LineType, VariableCollection};

use super::{builder::build_env};

use once_cell::sync::Lazy;

pub static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"#\[(\w+)\]\s*=?\s*(?:"([^"]*)"|(.*))$"#).unwrap());

pub static ENV_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\s*([A-Za-z0-9_]+)\s*=\s*(?:"([\s\S]*)"|([\s\S]*))$"#).unwrap());

pub static COMMENT_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*#.*$").unwrap());

pub fn parse_variables(env_file_content: &Vec<FileLine>, template: bool) -> io::Result<VariableCollection>
{
  let mut environment_variables: VariableCollection = VariableCollection::new();
  let mut current_env: HashMap<String, (String, u16)> = HashMap::new();

  let mut line_index: u16 = 1;
  for line in env_file_content
  {
    line_index = line.index;
    let line = line.line.as_str();
    let linetype = check_line(&line);
    if linetype == LineType::Empty || linetype == LineType::Comment || linetype == LineType::Invalid
    {
      if linetype == LineType::Invalid
      {
        emit_error!("Invalid syntax at line: {}", line_index);
      }

      if current_env.is_empty()
      {
        continue;
      }
      else 
      {  
        let env = build_env(&current_env, line_index-1);
        
        if environment_variables.contains_key(&env.key)
        {
          emit_warn!("Variable '{}' is defined more than once at line {}", env.key, line_index-1);
          environment_variables.remove(&env.key);
        }

        environment_variables.add(env);
        current_env.clear();
      }
    }
    else if linetype == LineType::Tag
    {
      if template
      {
        let tag: VariableMetadata = parse_tag_line(&line, line_index);
        current_env.insert(tag.name.to_string(), (tag.value, line_index));
      }
    }
    else if linetype == LineType::Env
    {
      let (env_key, env_value) = parse_env_line(&line, line_index);

      current_env.insert("key".to_string(), (env_key, line_index));
      current_env.insert("value".to_string(), (env_value, line_index));
    }
  }

  // process the last record, if current_record is not empty
  if !current_env.is_empty()
  {
    
    let env = build_env(&current_env, line_index-1);
        
    if environment_variables.contains_key(&env.key)
    {
      emit_warn!("Variable '{}' is defined more than once at line {}", env.key, line_index-1);
      environment_variables.remove(&env.key);
    }

    environment_variables.add(env);
  }

  return Ok(environment_variables);
}



pub fn check_line(mut line: &str) -> LineType
{
  line = line.trim();
  if line.is_empty()
  {
    return LineType::Empty;
  }
  
  if TAG_MATCHER.is_match(line)
  {
    return LineType::Tag;
  }

  if ENV_MATCHER.is_match(line)
  {
    return LineType::Env;
  }

  if COMMENT_MATCHER.is_match(line.trim())
  {
    return LineType::Comment;
  }

  return LineType::Invalid
}


pub fn parse_tag_line(line: &str, line_index: u16) -> VariableMetadata
{
  let captures = TAG_MATCHER.captures(&line);

  if let Some(captures) = captures
  {
    let (metadata_tag, value) = parse_tag_matcher(captures, line_index);
 
    return VariableMetadata { name: metadata_tag, value: value.to_string() };
  } 
  else 
  {
    emit_error!("Invalid syntax at line: {}", line_index);
  }
}

pub fn parse_env_line(line: &str, line_index: u16) -> (String, String)
{
  let captures = ENV_MATCHER.captures(&line);
  if captures.is_some()
  {
    let captures_unwrapped = captures;

    if captures_unwrapped.is_none()
    {
      emit_error!("Invalid syntax at line: {}", line_index);
    }

    let captures = captures_unwrapped.unwrap();
    
    let env_key: String = captures.get(1).map_or("", |m| m.as_str()).to_string();

    if env_key == ""
    {
      emit_error!("Invalid syntax at line: {}", line_index);
    }
    
    let mut env_value: String = captures.get(2).map_or("", |m| m.as_str()).to_string();

    if env_value == ""
    {
      env_value = captures.get(3).map_or("", |m| m.as_str()).to_string();
    }

    return (env_key, env_value);
  }
  return ("".to_string(), "".to_string());
}

fn parse_tag_matcher(captures: Captures<'_>, line_index: u16) -> (VariableTag, &str)
{

  let tag: &str = captures.get(1).map_or("", |m| m.as_str());

  let mut value: &str = captures.get(2).map_or("", |m| m.as_str());

  if value == ""
  {
    value = captures.get(3).map_or("", |m| m.as_str());
  }

  match VariableTag::from_str(tag)
  {
    Ok(tag) =>
    {
      return (tag, value.trim());
    },
    Err(_) => 
    {
      emit_error!("Invalid tag '{}' at line {}", tag, line_index);
    },
  }
}