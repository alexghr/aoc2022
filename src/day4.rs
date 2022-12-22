use std::str::FromStr;

use crate::util::{read_file, ParseAoCError};

pub fn run() {
  let data = read_file("data/day04.txt");
  part1(&data);
  part2(&data);
}

struct SectionRange {
  start: u32,
  end: u32
}

impl FromStr for SectionRange {
  type Err = ParseAoCError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split("-").collect();
    match (parts.first(), parts.last()) {
      (Some(start_str), Some(end_str)) => match (start_str.parse(), end_str.parse()) {
        (Ok(start), Ok(end)) => Ok(SectionRange { start, end }),
        _ => Err(ParseAoCError::new("SectionRange", s))
      },
      _ => Err(ParseAoCError::new("SectionRange", s))
    }
  }
}

impl SectionRange {
  pub fn is_contained_by(&self, other: &SectionRange) -> bool {
    self.start >= other.start && self.end <= other.end
  }

  pub fn is_partially_overlapping(&self, other: &SectionRange) -> bool {
    (self.start >= other.start && self.start <= other.end)
    || (self.end >= other.start && self.end <= other.end)
  }
}

struct Pair {
  first: SectionRange,
  second: SectionRange
}

impl FromStr for Pair {
  type Err = ParseAoCError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(",").collect();
    match (parts.first(), parts.last()) {
      (Some(start_str), Some(end_str)) => match (start_str.parse(), end_str.parse()) {
        (Ok(first), Ok(second)) => Ok(Pair { first, second }),
        _ => Err(ParseAoCError::new("Pair", s))
      },
      _ => Err(ParseAoCError::new("Pair", s))
    }
  }
}

impl Pair {
  pub fn fully_overlaps(&self) -> bool {
    self.first.is_contained_by(&self.second) ||
    self.second.is_contained_by(&self.first)
  }

  pub fn overlaps(&self) -> bool {
    self.first.is_partially_overlapping(&self.second) ||
    self.second.is_partially_overlapping(&self.first)
  }
}

fn part1(data: &Vec<String>) {
  let parsed: Vec<Pair> = data.into_iter().map(|x| x.parse().unwrap()).collect();
  let count = parsed.into_iter().filter(|x| x.fully_overlaps()).count();

  println!("Number of fully overlapping pairs is: {}", count);
}

fn part2(data: &Vec<String>) {
  let parsed: Vec<Pair> = data.into_iter().map(|x| x.parse().unwrap()).collect();
  let count = parsed.into_iter().filter(|x| x.overlaps()).count();

  println!("Number of partially overlapping pairs is: {}", count);
}
