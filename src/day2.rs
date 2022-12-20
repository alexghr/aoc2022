use std::{error::Error, fmt::Display, str::FromStr};

use crate::util::read_file;

enum Shape {
  Rock,
  Paper,
  Scissor
}

#[derive(Debug)]
struct ParseShapeError {
  val: String
}

impl Error for ParseShapeError {}
impl Display for ParseShapeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   write!(f, "Failed to parse {} into Shape", self.val)
  }
}

impl Shape {
  pub fn value(&self) -> u32 {
    match self {
      Shape::Rock => 1,
      Shape::Paper => 2,
      Shape::Scissor => 3,
    }
  }
}

impl FromStr for Shape {
  type Err = ParseShapeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      // opponent's hand
      "A" => Ok(Shape::Rock),
      "B" => Ok(Shape::Paper),
      "C" => Ok(Shape::Scissor),

      // my hand
      "X" => Ok(Shape::Rock),
      "Y" => Ok(Shape::Paper),
      "Z" => Ok(Shape::Scissor),
      _ => Err(ParseShapeError { val: s.to_string() })
    }
  }
}
enum Outcome {
  Win,
  Draw,
  Loss
}

impl Outcome {
  pub fn value(&self) -> u32 {
    match self {
      Outcome::Win => 6,
      Outcome::Draw => 3,
      Outcome::Loss => 0
    }
  }
}


#[derive(Debug)]
struct ParseRoundError {
  val: String
}
impl Error for ParseRoundError {}

impl Display for ParseRoundError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "Failed to parse round: {}", self.val)
  }
}

struct Round {
  me: Shape,
  opp: Shape
}

impl Round {
  pub fn outcome(&self) -> Outcome {
    match (&self.me, &self.opp) {
      (Shape::Rock, Shape::Paper) => Outcome::Loss,
      (Shape::Rock, Shape::Scissor) => Outcome::Win,
      (Shape::Rock, Shape::Rock) => Outcome::Draw,

      (Shape::Paper, Shape::Paper) => Outcome::Draw,
      (Shape::Paper, Shape::Scissor) => Outcome::Loss,
      (Shape::Paper, Shape::Rock) => Outcome::Win,

      (Shape::Scissor, Shape::Paper) => Outcome::Win,
      (Shape::Scissor, Shape::Scissor) => Outcome::Draw,
      (Shape::Scissor, Shape::Rock) => Outcome::Loss,
    }
  }

  pub fn value(&self) -> u32 {
    self.me.value() + self.outcome().value()
  }
}

impl FromStr for Round {
  type Err = ParseRoundError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let r: Vec<&str> = s.split_whitespace().collect();
    let opp_res = r.first().unwrap().parse();
    let me_res = r.last().unwrap().parse();

    match (opp_res, me_res) {
      (Ok(opp), Ok(me)) => Ok(Round { me, opp }),
      (_, _) => Err(ParseRoundError { val: s.to_string() })
    }
  }
}

pub fn run() {
  let data = parse_data(&read_file("data/day2.txt"));
  part1(&data);
}

fn part1(data: &Vec<Round>) {
  let score = data.into_iter().fold(0, |s, r| s + r.value());
  println!("Strategy guide total score: {}", score);
}

fn parse_data(data: &Vec<String>) -> Vec<Round> {
  data.into_iter().map(|line| line.parse().unwrap()).collect()
}
