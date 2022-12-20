use std::fs;

pub fn read_file(path: &str) -> Vec<String> {
  fs::read_to_string(path)
      .unwrap()
      .split_terminator('\n')
      .map(|line| String::from(line))
      .collect()
}
