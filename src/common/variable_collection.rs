use crate::emit_warn;

use super::Variable;

pub struct VariableCollection 
{
  variables: Vec<Variable>,
}

impl VariableCollection 
{
  pub fn new() -> Self
  {
    VariableCollection {
      variables: Vec::new(),
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = &Variable>
  {
    self.variables.iter()
  }

  pub fn add(&mut self, value: Variable, line_index: u16)
  {
    if self.contains_key(&value.key) {
      emit_warn!("Variable '{}' is defined more than once at line {}", value.key, line_index);
      self.remove(&value.key);
    }
    self.variables.push(value);
  }

  pub fn get(&self, key: &str) -> Option<&Variable>
  {
    self.variables.iter().find(|x| x.key == key)
  }

  pub fn contains_key(&self, key: &str) -> bool
  {
    for env in self.variables.iter()
    {
      if env.key == key
      {
        return true;
      }
    }
    return false;
  }

  pub fn remove(&mut self, key: &str)
  {
    if let Some(index) = self.variables.iter().position(|env| env.key == key)
    {
      self.variables.remove(index);
    }
  }
}