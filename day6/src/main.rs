use std::{collections::HashMap, fs};

fn main() {
  let contents = fs::read_to_string("./input.txt")
    .expect("Cannot load file.");

  let group_answers: Vec<&str> = contents.split("\n\n").collect();
  let mut sum: u32 = 0;

  group_answers
    .iter()
    .for_each(|answers|
      sum += count_occurrences(answers.trim())
    );
  println!("{} answered questions", sum);
}

fn count_occurrences(group_answers: &str) -> u32 {
  let answers: Vec<&str> = group_answers.split("\n").collect();
  let mut counts: HashMap<char, u32> = HashMap::new();
  let group_size = answers.len() as u32;

  for answer in answers.iter() {
    let chars: Vec<char> = answer.chars().collect();

    for char in chars.iter() {
      if *char == '\n' {
        continue;
      }

      let entry = counts.entry(*char).or_insert(0);
      *entry += 1;
    }
  }

  counts
    .iter()
    .filter(|(_k, v)| *v == &group_size)
    .fold(0, |acc, (_k, _v)| acc + 1)
}
