#[derive(Debug)]
pub struct FileLine
{
  pub index: u16,
  pub line: String,
}

#[derive(PartialEq, Debug)]
pub enum LineType
{
  Empty,
  Tag,
  Env,
  Comment,
  Invalid
}