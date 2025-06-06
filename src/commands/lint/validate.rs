use std::i128;

use regex::Regex;

use crate::emit_error;

use crate::common::{VariableCollection, VariableRangeList, VariableType};

pub fn validate_variable(
  env_filename: &str, 
  environment_variables: &VariableCollection, 
  template_environment_variables: &VariableCollection
) {
  for variable in template_environment_variables.iter()
  {
    let variable_value_unwrapped = environment_variables.get(&variable.key);

    let variable_value = match variable_value_unwrapped
    {
      Some(variable) => variable.value.clone(),
      None =>
      {
        "".to_string()
      },
    };

    if variable_value.trim() == ""
    {
      if variable.required == true
      {
        emit_error!("missing value for key '{}' in file '{}'", variable.key, env_filename);
      }
      continue;
    }

    if variable.env_type == VariableType::Boolean
    {
      validate_boolean_value(&variable_value, &variable.key);
    }
    else if variable.env_type == VariableType::String
    {
      validate_string_value(&variable_value, &variable.regex, &variable.key);
    }
    else if variable.env_type == VariableType::Integer
    {
      validate_integer(&variable_value, &variable.range, &variable.key);
    }
    else if variable.env_type == VariableType::Float
    {
      validate_float(&variable_value, &variable.range, &variable.key);
    }
  }
}

/// Validate the boolean value
fn validate_boolean_value(value: &str, key: &str)
{
  let normalized = value.trim().to_ascii_lowercase();
  let parsed = match normalized.as_str() {
      "1" | "t" | "true" => Ok(true),
      "0" | "f" | "false" => Ok(false),
      _ => Err(()),
  };

  if parsed.is_err() {
      emit_error!("value for key '{}' is not a valid boolean. expected '1', 't', 'T', 'TRUE', 'true', 'True', '0', 'f', 'F', 'FALSE', 'false', 'False'", key);
  }
}

/// Valodate the string value
fn validate_string_value(value: &str, regex: &Regex, key: &str)
{
  if regex.as_str() != ""
  {
    if !regex.is_match(value)
    {
      emit_error!("value for key '{}' does not match the regex '{}'", key, regex.as_str());
    }
  }
}

/// Validate the integer value
fn validate_integer(value: &str, range_list: &VariableRangeList, key: &str)
{
  let integer_value = match value.parse::<i128>()
  {
    Ok(value) => value,
    Err(_) =>
    {
      emit_error!("value for key '{}' is not an integer", key);
    },
  };
  if range_list.ranges.len() > 0 
  {
    let mut is_in_range = false;
    for range in range_list.ranges.iter()
    {
      let min: i128 = range.min.parse().unwrap();
      let max: i128 = range.max.parse().unwrap();

      if integer_value >= min && integer_value <= max
      {
        is_in_range = true;
        break;
      }
    }
  
    if is_in_range == false
    {
      emit_error!("value for key '{}' is not in the range '{}'", key, range_list.raw);
    }
  }
}

/// Validate the float value
fn validate_float(value: &str, range_list: &VariableRangeList, key: &str)
{
  let float_value = match value.parse::<f64>()
  {
    Ok(value) => value,
    Err(_) =>
    {
      emit_error!("value for key '{}' is not a float", key);
    },
  };

  if range_list.ranges.len() > 0
  {
    let mut in_range_status = false;
    for range in range_list.ranges.iter()
    {
      let min: f64 = range.min.parse().unwrap();
      let max: f64 = range.max.parse().unwrap();

      if float_value >= min && float_value <= max
      {
        in_range_status = true;
        break;
      }
    }
  
    if in_range_status == false
    {
      emit_error!("value for key '{}' is not in the range '{}'", key, range_list.raw);
    }
  }
}