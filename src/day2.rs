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

  pub fn compare(&self, other: &Shape) -> Outcome {
    match (&self, &other) {
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

  // what does the other play need to play to achieve this outcome
  // (from the current player's perspective)
  pub fn for_outcome(&self, outcome: &Outcome) -> Shape {
    match (self, outcome) {
      (Shape::Rock, Outcome::Win) => Shape::Scissor,
      (Shape::Rock, Outcome::Loss) => Shape::Paper,
      (Shape::Rock, Outcome::Draw) => Shape::Rock,

      (Shape::Paper, Outcome::Win) => Shape::Rock,
      (Shape::Paper, Outcome::Loss) => Shape::Scissor,
      (Shape::Paper, Outcome::Draw) => Shape::Paper,

      (Shape::Scissor, Outcome::Win) => Shape::Paper,
      (Shape::Scissor, Outcome::Loss) => Shape::Rock,
      (Shape::Scissor, Outcome::Draw) => Shape::Scissor,
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

      // my hand. Part 1 only
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

  pub fn reverse(&self) -> Outcome {
    match self {
      Outcome::Win => Outcome::Loss,
      Outcome::Loss => Outcome::Win,
      Outcome::Draw => Outcome::Draw
    }
  }
}

#[derive(Debug)]
struct ParseOutcomeError {
  val: String
}

impl Error for ParseOutcomeError {}
impl Display for ParseOutcomeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   write!(f, "Failed to parse {} into outcome", self.val)
  }
}

impl FromStr for Outcome {
  type Err = ParseOutcomeError;

  // Part 2 only
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "X" => Ok(Outcome::Loss),
      "Y" => Ok(Outcome::Draw),
      "Z" => Ok(Outcome::Win),
      _ => Err(ParseOutcomeError { val: s.to_string() })
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
  pub fn value(&self) -> u32 {
    self.me.value() + self.me.compare(&self.opp).value()
  }

  pub fn from_p1_strategy(s: &str) -> Result<Self, ParseRoundError> {
    let r: Vec<&str> = s.split_whitespace().collect();
    let opp_res = r.first().unwrap().parse();
    let me_res = r.last().unwrap().parse();

    match (opp_res, me_res) {
      (Ok(opp), Ok(me)) => Ok(Round { me, opp }),
      (_, _) => Err(ParseRoundError { val: s.to_string() })
    }
  }

  pub fn from_p2_strategy(s: &str) -> Result<Self, ParseRoundError> {
    let r: Vec<&str> = s.split_whitespace().collect();
    let opp_res: Result<Shape, ParseShapeError> = r.first().unwrap().parse();
    let outcome_res: Result<Outcome, ParseOutcomeError> = r.last().unwrap().parse();

    match (opp_res, outcome_res) {
      (Ok(opp), Ok(outcome)) => Ok(Round {
        // I need to reverse the outcome here. _I_ want to win, not my opponent
        me: opp.for_outcome(&outcome.reverse()),
        opp
      } ),
      (_, _) => Err(ParseRoundError { val: s.to_string() })
    }
  }
}

pub fn run() {
  let data = read_file("data/day2.txt");
  part1(&data);
  part2(&data);
}

fn part1(data: &Vec<String>) {
  let score = data.into_iter().map(|s| Round::from_p1_strategy(s).unwrap()).fold(0, |s, r| s + r.value());
  println!("Imperfect strategy guide total score: {}", score);
}

fn part2(data: &Vec<String>) {
  let score = data.into_iter().map(|s| Round::from_p2_strategy(s).unwrap()).fold(0, |s, r| s + r.value());
  println!("Strategy guide total score: {}", score);
}
