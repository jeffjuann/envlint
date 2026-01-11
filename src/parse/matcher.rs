use once_cell::sync::Lazy;

use regex::Regex;

pub static TAG_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^#\[(\w+)\]\s*=?\s*(?:"([^"]*)"|(.*))$"#).unwrap());

pub static ENV_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\s*([A-Za-z0-9_]+)\s*=\s*(?:"([\s\S]*)"|([\s\S]*))$"#).unwrap());

pub static COMMENT_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*#.*$").unwrap());

pub static RANGE_MATCHER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(\d+(\.\d+)?(\.\.\d+(\.\d+)?)?)(,(\d+(\.\d+)?(\.\.\d+(\.\d+)?)?))*$"#).unwrap());