
use std::str::FromStr;

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
}

impl FromStr for VariableTag
{
  type Err = ();

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
      _ => Err(()),
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
  Unknown,
}

impl FromStr for VariableType
{
  type Err = ();

  fn from_str(value: &str) -> Result<Self, Self::Err>
  {
    match value
    {
      "string" => Ok(VariableType::String),
      "integer" => Ok(VariableType::Integer),
      "float" => Ok(VariableType::Float),
      "boolean" => Ok(VariableType::Boolean),
      _ => Err(()),
    }
  }
}