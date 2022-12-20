use std::cmp::Ordering;

use crate::util;

pub fn run() {
  let data = parse(&util::read_file("data/day1.txt"));
  part_1(&data);
  part_2(&data);
}

fn part_1(data: &Vec<Vec<u32>>) {
  let &max = sums_sorted(data).first().unwrap();
  println!("Most calories carried: {}", max);
}

fn part_2(data: &Vec<Vec<u32>>) {
  let sums = sums_sorted(data);
  let top3_total = &sums[0..3].into_iter().fold(0, |s, x| s + x);
  println!("Calories carried by top 3 elves: {}", top3_total);
}

fn sums_sorted(data: &Vec<Vec<u32>>) -> Vec<u32> {
  let mut sums: Vec<u32> = data
    .into_iter()
    .map(|x| x.into_iter().fold(0, |sum, y| sum + y))
    .collect();

  sums.sort_by(|a, b| {
    if a < b {
      Ordering::Greater
    } else if a == b {
      Ordering::Equal
    } else {
      Ordering::Less
    }
  });

  sums
}

fn parse(data: &Vec<String>) -> Vec<Vec<u32>> {
  data.into_iter().fold(vec![vec![]], |mut acc, x| {
    let last = acc.last_mut().unwrap();
    match x.parse::<u32>() {
      Ok(val) => last.push(val),
      Err(_) => acc.push(vec![])
    };

    acc
  })
}
