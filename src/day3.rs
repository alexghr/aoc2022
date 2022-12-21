use crate::util::read_file;

pub fn run() {
  let data = read_file("data/day03.txt");
  part1(&data);
  part2(&data);
}

fn part1(data: &Vec<String>) {
  let total = data.into_iter().fold(0, |total, pack| {
    let chars: Vec<char> = pack.chars().collect();
    let (first, second) = compartmentalize(&chars);
    total + all_items()
      .into_iter()
      .filter(|item| first.contains(item) && second.contains(item))
      .fold(0, |s, x| s + priority(&x))
  });

  println!("Total cost of duplicate items: {}", total)
}

fn part2(data: &Vec<String>) {
  let sum = data.into_iter().step_by(3)
    .zip(data.into_iter().skip(1).step_by(3))
    .zip(data.into_iter().skip(2).step_by(3))
    .map(|((a, b), c)| (a,b,c))
    .fold(0, |s, (a, b, c)| {
      s + priority(
        &all_items()
          .into_iter()
          .find(|&x| a.contains(x) && b.contains(x) && c.contains(x))
          .unwrap()
        )
    });

  println!("Total cost of badges: {}", sum);
}

fn compartmentalize(items: &[char]) -> (&[char], &[char]) {
  items.split_at(items.len() / 2)
}

fn all_items() -> Vec<char> {
  // const lower_case: Vec<char> = ('a'..'z').collect();
  // const upper_case: Vec<char> = ('A'..'Z').collect();

  ('a'..='z').chain('A'..='Z').collect()
}

fn priority(item: &char) -> u32 {
  1 + u32::try_from(
    all_items()
    .into_iter()
    .position(|x| x == *item)
    .unwrap()
  ).unwrap()
}
