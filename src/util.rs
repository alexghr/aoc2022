use std::{fs, error::Error, fmt::Display};

pub fn read_file(path: &str) -> Vec<String> {
  fs::read_to_string(path)
      .unwrap()
      .split_terminator('\n')
      .map(|line| String::from(line))
      .collect()
}

#[derive(Debug)]
pub struct ParseAoCError {
  what: String,
  line: String
}

impl ParseAoCError {
  pub fn new(what: &str, line: &str) -> ParseAoCError {
    ParseAoCError { what: what.to_string(), line: line.to_string() }
  }
}

impl Error for ParseAoCError {}

impl Display for ParseAoCError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Failed to parse line as {}: {}", self.what, self.line)
  }
}
