use crate::util::read_file;

pub fn run() {
  let data = read_file("data/day03.txt");
  part1(&data);
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

  println!("Total cost: {}", total)
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
