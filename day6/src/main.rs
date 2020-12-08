use std::{collections::HashMap, fs, str::Chars};

fn main() {
  let contents = fs::read_to_string("./input.txt")
    .expect("Cannot load file.");

  let group_answers: Vec<&str> = contents.split("\n\n").collect();
  let mut sum: u32 = 0;

  group_answers
    .iter()
    .for_each(|answers|
      sum += count_occurrences(answers.trim().chars())
    );

  println!("{} answered questions", sum);
}

fn count_occurrences(answers: Chars) -> u32 {
  let chars: Vec<char> = answers.collect();
  let mut counts: HashMap<char, u32> = HashMap::new();

  for char in chars.iter() {
    if *char == '\n' {
      continue;
    }

    counts.entry(*char).or_insert(1);
  }

  counts.values().fold(0, |acc, count| acc + count)
}
