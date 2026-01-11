
use std::{str::FromStr, fmt};

use regex::Regex;

#[derive(Clone, Debug)]
pub struct Variable
{
  /// Environment variable key (ex: PORT, DATABASE_URL, etc)
  pub key: String,
  /// Environment variable title
  pub title: String,
  /// Environment variable description
  pub description: String,
  /// Environment variable required status
  pub required: bool,
  /// Environment variable type
  pub env_type: VariableType,
  /// Environment variable regex
  pub regex: Regex,
  /// Environment variable range
  pub range: VariableRangeList,
  /// Environment variable default value
  pub default_value: String,
  /// Environment variable value
  pub value: String,
}

impl Variable
{
  pub fn new() -> Self
  {
    Variable {
      key: String::new(),
      title: String::new(),
      description: String::new(),
      required: false,
      env_type: VariableType::Unknown,
      regex: Regex::new("").unwrap(),
      range: VariableRangeList::new(),
      default_value: String::new(),
      value: String::new(),
    }
  }

  pub fn set_key(&mut self, key: String) -> ()
  {
    self.key = key;
  }

  pub fn set_value(&mut self, value: String) -> ()
  {
    self.value = value;
  }

  pub fn set_title(&mut self, title: String) -> ()
  {
    self.title = title;
  }

  pub fn set_description(&mut self, description: String) -> ()
  {
    self.description = description;
  }

  pub fn set_required(&mut self, required: bool) -> ()
  {
    self.required = required;
  }

  pub fn set_env_type(&mut self, env_type: VariableType) -> ()
  {
    self.env_type = env_type;
  }

  pub fn set_regex(&mut self, regex: Regex) -> ()
  {
    self.regex = regex;
  }

  pub fn set_range(&mut self, range: VariableRangeList) -> ()
  {
    self.range = range;
  }

  pub fn set_default_value(&mut self, default_value: String) -> ()
  {
    self.default_value = default_value;
  }
}

#[derive(Clone, Debug)]
pub struct VariableRangeList
{
  pub raw: String,
  pub ranges: Vec<VariableRange>,
}

impl VariableRangeList
{
  pub fn new() -> Self
  {
    VariableRangeList {
      raw: String::new(),
      ranges: Vec::new(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct VariableRange
{
  pub min: String,
  pub max: String,
}

impl VariableRange
{
  pub fn new(min: String, max: String) -> Self
  {
    VariableRange {
      min,
      max,
    }
  }
}

pub struct VariableMetadata
{
  pub name: VariableTag,
  pub value: String
}

#[derive(PartialEq)]
pub enum VariableTag
{
  Title,
  Description,
  Required,

  Type,
  Regex,
  Range,
  Default,
}

pub enum VariableTagErrorKind
{
  Unknown(String),
}

impl FromStr for VariableTag
{
  type Err = VariableTagErrorKind;

  fn from_str(value: &str) -> Result<Self, Self::Err>
  {
    match value
    {
      "title" => Ok(VariableTag::Title),
      "description" => Ok(VariableTag::Description),
      "required" => Ok(VariableTag::Required),
      "type" => Ok(VariableTag::Type),
      "regex" => Ok(VariableTag::Regex),
      "range" => Ok(VariableTag::Range),
      "default" => Ok(VariableTag::Default),
      _ => Err(VariableTagErrorKind::Unknown(value.to_string())),
    }
  }
}

impl ToString for VariableTag
{
  fn to_string(&self) -> String
  {
    match self
    {
      VariableTag::Title => String::from("title"),
      VariableTag::Description => String::from("description"),
      VariableTag::Required => String::from("required"),
      VariableTag::Type => String::from("type"),
      VariableTag::Regex => String::from("regex"),
      VariableTag::Range => String::from("range"),
      VariableTag::Default => String::from("default"),
    }
  }
}

#[derive(PartialEq, Clone, Debug)]
pub enum VariableType
{
  String,
  Integer,
  Float,
  Boolean,
  List(Box<VariableType>),
  Unknown,
}

#[derive(PartialEq)]
pub enum VariableTypeErrorKind
{
  Unknown(String),
  DoubleNestedList
}

impl VariableType
{
  fn from_str_with_depth(value: &str, original_value: &str, depth: usize) -> Result<Self, VariableTypeErrorKind>
  {
    match value
    {
      "string" => Ok(VariableType::String),
      "integer" => Ok(VariableType::Integer),
      "float" => Ok(VariableType::Float),
      "boolean" => Ok(VariableType::Boolean),
      _ => {
        match value.strip_prefix("list<").and_then(|v| v.strip_suffix(">"))
        {
          Some(inner) =>
          {
            if depth > 0
            {
              Err(VariableTypeErrorKind::DoubleNestedList)
            } 
            else 
            {
              Ok(VariableType::List(
                Box::new(Self::from_str_with_depth(inner, original_value, depth + 1)?)
              ))
            }
          },
          None => Err(VariableTypeErrorKind::Unknown(original_value.to_string()))
        }
      },
    }
  }
}


impl FromStr for VariableType
{
  type Err = VariableTypeErrorKind;

  fn from_str(value: &str) -> Result<Self, Self::Err>
  {
    Self::from_str_with_depth(value, value, 0)
  }
}

impl fmt::Display for VariableType 
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  {
    match self
    {
      VariableType::String => write!(f, "string"),
      VariableType::Boolean => write!(f, "boolean"),
      VariableType::Integer => write!(f, "integer"),
      VariableType::Float => write!(f, "float"),
      VariableType::List(generic) => write!(f, "list of {}s", generic),
      VariableType::Unknown => write!(f, "unknown"),
    }
  }
}