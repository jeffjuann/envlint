use std::i128;

use regex::Regex;

use crate::emit_error;

use crate::common::{Variable, VariableCollection, VariableRangeList, VariableType};

#[derive(Clone, Copy)]
pub enum ValueKind {
  Value,
  DefaultValue,
}

impl ValueKind {
  fn as_str(&self) -> &'static str {
    match self {
      ValueKind::Value => "value",
      ValueKind::DefaultValue => "default value",
    }
  }
}

pub fn validate_variable(
  env_filename: &str, 
  environment_variables: &VariableCollection, 
  template_environment_variables: &VariableCollection
) {
  for variable in template_environment_variables.iter()
  {
    if !variable.default_value.trim().is_empty()
    {
      validate_value(variable, &variable.default_value, ValueKind::DefaultValue);
    }

    let variable_value = match environment_variables.get(&variable.key)
    {
      Some(v) if !v.value.trim().is_empty() => &v.value,
      _ => 
      {
        if variable.required == true
        {
          emit_error!("missing value for key '{}' in file '{}'", variable.key, env_filename);
        }
        continue;
      }
    };

    validate_value(variable, variable_value, ValueKind::Value);
  }
}

pub fn validate_value(
  template_variable: &Variable,
  value: &String,
  value_kind: ValueKind
) {
  match &template_variable.env_type
  {
    VariableType::Boolean => validate_boolean(&value, &template_variable.env_type, &template_variable.key, value_kind),
    VariableType::String => validate_string(&value, &template_variable.regex, &template_variable.env_type, &template_variable.key, value_kind),
    VariableType::Integer => validate_integer(&value, &template_variable.range, &template_variable.env_type, &template_variable.key, value_kind),
    VariableType::Float => validate_float(&value, &template_variable.range, &template_variable.env_type, &template_variable.key, value_kind),
    VariableType::List(generic) => validate_list(template_variable, generic, value, value_kind),
    _ => {},
  }
}

fn validate_list(
  template_variable: &Variable,
  generic: &VariableType,
  value: &String,
  value_kind: ValueKind
) {
  for v in value.split(',')
  {
    match &generic
    {
      VariableType::Boolean => validate_boolean(v, &template_variable.env_type, &template_variable.key, value_kind),
      VariableType::String => validate_string(v, &template_variable.regex, &template_variable.env_type, &template_variable.key, value_kind),
      VariableType::Integer => validate_integer(v, &template_variable.range, &template_variable.env_type, &template_variable.key, value_kind),
      VariableType::Float => validate_float(v, &template_variable.range, &template_variable.env_type, &template_variable.key, value_kind),
      VariableType::List(generic) => validate_list(template_variable, generic, value, value_kind),
      _ => {},
    }
  }
}

/// Validate the boolean value
fn validate_boolean(value: &str, original_type: &VariableType, key: &str, value_kind: ValueKind)
{
  let normalized = value.trim().to_ascii_lowercase();
  let parsed = match normalized.as_str() {
    "1" | "t" | "true" => Ok(true),
    "0" | "f" | "false" => Ok(false),
    _ => Err(()),
  };

  if parsed.is_err() {
    emit_error!("{} for key '{}' is not a valid {}. expected '1', 't', 'T', 'TRUE', 'true', 'True', '0', 'f', 'F', 'FALSE', 'false', 'False'", value_kind.as_str(), key, original_type);
  }
}

/// Valodate the string value
fn validate_string(value: &str, regex: &Regex, _original_type: &VariableType, key: &str, value_kind: ValueKind)
{
  if !regex.as_str().is_empty()
  {
    if !regex.is_match(value)
    {
      emit_error!("{} for key '{}' does not match the regex '{}'", value_kind.as_str(), key, regex.as_str());
    }
  }
}

/// Validate the integer value
fn validate_integer(value: &str, range_list: &VariableRangeList, original_type: &VariableType, key: &str, value_kind: ValueKind)
{
  let integer_value = match value.parse::<i128>()
  {
    Ok(value) => value,
    Err(_) =>
    {
      emit_error!("{} for key '{}' is not a valid {}", value_kind.as_str(), key, original_type);
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
      emit_error!("{} for key '{}' is not in the range '{}'", value_kind.as_str(), key, range_list.raw);
    }
  }
}

/// Validate the float value
fn validate_float(value: &str, range_list: &VariableRangeList, original_type: &VariableType, key: &str, value_kind: ValueKind)
{
  let float_value = match value.parse::<f64>()
  {
    Ok(value) => value,
    Err(_) =>
    {
      emit_error!("{} for key '{}' is not a valid {}", value_kind.as_str(), key, original_type);
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
      emit_error!("{} for key '{}' is not in the range '{}'", value_kind.as_str(), key, range_list.raw);
    }
  }
}