use std::{collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;

use regex::Regex;

use crate::{emit_error, emit_info, emit_warn};

use crate::common::{Variable, VariableType, VariableRangeList, VariableRange};

pub static RANGE_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(\d+(\.\d+)?(\.\.\d+(\.\d+)?)?)(,(\d+(\.\d+)?(\.\.\d+(\.\d+)?)?))*$"#).unwrap());

pub fn build_env(variable: &HashMap<String, (String, u16)>, line_index: u16) -> Variable
{
  let env_key: &str = match variable.get("key")
  {
    Some((key_str, line_index)) => 
    {
      if key_str.is_empty()
      {
        emit_error!("Variable key is empty at line {}", line_index);
      }
      key_str
    },
    None =>
    {
      emit_error!("Variable key is missing at line {}", line_index);
    }
  };

  let mut env: Variable = Variable::new();

  env.set_key(env_key.to_string());

  env.set_value(match variable.get("value")
  {
    Some((value, _)) => value.to_string(),
    None => String::new()
  });

  // if variable only has a key and value, skip the rest of the checks
  if variable.len() == 2 && variable.contains_key("key") && variable.contains_key("value")
  {
    return env;
  }

  env.set_title(match variable.get("title")
  {
    Some((title_str, line_index)) => 
    {
      if title_str.is_empty()
      {
        emit_warn!("Variable '{}' has an empty title at line {}", env.key, line_index);
      }
      title_str.to_string()
    },
    None =>
    {
      emit_warn!("Variable '{}' is missing a title", env.key);
      String::new()
    }
  });

  env.set_description(match variable.get("description")
  {
    Some((description_str, line_index)) => 
    {
      if description_str.is_empty()
      {
        emit_warn!("Variable '{}' has an empty description at line {}", env.key, line_index);
      }
      description_str.to_string()
    },
    None =>
    {
      emit_warn!("Variable '{}' is missing a description", env.key);
      String::new()
    }
  });

  env.set_required(match variable.get("required")
  {
    Some((required_str, line_index)) => 
    {
      if required_str == "true" || required_str.is_empty()
      {
        true
      }
      else if required_str == "false"
      {
        false
      }
      else
      {
        emit_warn!("Variable '{}' has an invalid required value '{}' at line {}. expected 'true' or 'false', consider removing the flag if it's not needed.", env.key, required_str, line_index);
        true
      }
    },
    None => false
  });
  
  env.set_env_type(match variable.get("type")
  {
    Some((env_type_str, line_index)) => 
    {
      match VariableType::from_str(env_type_str)
      {
        Ok(env_type) => env_type,
        Err(_) => 
        {
          emit_error!("Variable '{}' has an unknown type '{}' at line {}", env.key, env_type_str, line_index);
        }
      }
    },
    None =>
    {
      emit_warn!("Variable '{}' is missing a type", env.key);
      VariableType::Unknown
    }
  });

  match variable.get("regex")
  {
    Some((regex_str, line_index)) =>
    {
      if env.env_type != VariableType::String
      {
        emit_warn!("Variable '{}' has a regex but is not a string type at line {}", env.key, line_index);
      }
      else 
      { 
        let regex = Regex::new(regex_str);
        match regex
        {
          Ok(regex) => env.set_regex(regex),
          Err(_) => 
          {
            emit_error!("Variable '{}' has an invalid regex '{}' at line {}", env.key, regex_str, line_index);
          }    
        }
      }
    },
    None => {}
  };

  env.set_range(match variable.get("range")
  {
    Some((range_str, line_index)) => process_range(&env, range_str, line_index),
    None => VariableRangeList::new()
  });

  env
}

fn process_range(env: &Variable, range_str: &str, line_index: &u16) -> VariableRangeList
{
  let mut range: VariableRangeList = VariableRangeList::new();
  if env.env_type != VariableType::Integer && env.env_type != VariableType::Float
  {
    emit_info!("Variable '{}' has a range but is not an 'integer' or 'float'", env.key);
    range
  }
  else if range_str.is_empty() 
  {
    emit_warn!("Variable '{}' has an empty range", env.key);
    range
  }
  else
  { 
    range.raw = range_str.to_string();

    if !RANGE_MATCHER.is_match(range_str) {
      emit_error!("Variable '{}' has an invalid range '{}' at line {}", env.key, range_str, line_index);
    }

    let range_parts: Vec<String> = range_str.split(',').map(|s| s.trim().to_string()).collect();
    if range_parts.is_empty() {
      emit_warn!("Variable '{}' has an empty range", env.key);
    }
    
    for range_part in range_parts
    {
      let range_values: Vec<&str> = range_part.split("..").collect();
      if range_values.len() == 1
      {
        // single value
        let value = range_values[0].trim();
        if value.is_empty()
        {
          emit_warn!("Variable '{}' has an empty range value", env.key);
        }
        else
        {
          range.ranges.push(VariableRange::new(value.to_string(), value.to_string()))
        }
      }
      else if range_values.len() == 2
      {
        // range
        let min = range_values[0].trim();
        let max = range_values[1].trim();
        if min.is_empty() || max.is_empty()
        {
          emit_warn!("Variable '{}' has an empty range value", env.key);
        }
        else
        {
          range.ranges.push(VariableRange::new(min.to_string(), max.to_string()));
        }
      }
      else
      {
        emit_error!("Variable '{}' has an invalid range '{}'", env.key, range_part);
      }
    }

    if range.ranges.is_empty() {
      emit_warn!("Variable '{}' has an empty range", env.key);
    }

    range
  }
}