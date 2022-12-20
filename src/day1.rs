use crate::util;

pub fn run() -> u32 {
  part_1(&util::read_file("data/day1.txt"))
}

fn part_1(data: &Vec<String>) -> u32 {
  data.into_iter().fold(vec![vec![]], |mut acc, x| {
    let last = acc.last_mut().unwrap();
    match x.parse::<u32>() {
      Ok(val) => last.push(val),
      Err(_) => acc.push(vec![])
    };

    acc
  }).into_iter().fold(0,|accum, x| {
    let sum = x.into_iter().fold(0, |sum, y| sum + y);
    if accum < sum {
      sum
    } else {
      accum
    }
  })
}
