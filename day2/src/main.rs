use std::fs;

fn main() {
  let data = fs::read_to_string(r#"./input.txt"#)
    .expect("Cannot load file.");

  let pairs: Vec<&str> = data.split("\n").collect();

  let valid_count =
    pairs
      .iter()
      .filter_map(|pair| is_valid_part_two(pair))
      .count();

  println!("Valid: {}", valid_count);
}

fn is_valid_part_two(pair: &str) -> Option<bool> {
  let chunks: Vec<&str> = pair.split(": ").collect();
  let policy = get_policy(chunks[0]);
  let password: &str = chunks[1];

  match policy {
    Policy::Invalid => None,
    Policy::Valid(first, second, letter) => {
      let chars: Vec<char> = password.chars().collect();
      let pos1 = first as usize - 1;
      let pos2 = second as usize - 1;

      if (chars[pos1] == letter) ^ (chars[pos2] == letter) {
        Some(true)
      } else {
        None
      }
    }
  }
}

fn is_valid(pair: &str) -> Option<bool> {
  let chunks: Vec<&str> = pair.split(": ").collect();
  let policy = get_policy(chunks[0]);
  let password: &str = chunks[1];

  match policy {
    Policy::Invalid => None,
    Policy::Valid(at_least, at_most, letter) => {
      let occurrences: Vec<&str> = password.matches(letter).collect();
      let count: u8 = occurrences.len() as u8;
      let range_arr: Vec<u8> = (at_least..at_most + 1).collect();

      if range_arr.contains(&count) {
        Some(true)
      } else {
        None
      }
    }
  }
}

fn get_policy(chunk: &str) -> Policy {
  let chunks: Vec<&str> = chunk.split(" ").collect();
  let range: Vec<&str> = chunks[0].split("-").collect();
  let mut at_least: Option<u8> = None;
  let mut at_most: Option<u8> = None;

  match range[0].parse() {
    Ok(n) => {
      at_least = Some(n)
    },
    Err(_) => {}
  }

  match range[1].parse() {
    Ok(n) => {
      at_most = Some(n)
    },
    Err(_) => {}
  }

  let letter = chunks[1].chars().nth(0).unwrap();

  if at_least == None || at_most == None {
    Policy::Invalid
  } else {
    Policy::Valid(at_least.unwrap(), at_most.unwrap(), letter)
  }
}

enum Policy {
  Valid(u8, u8, char),
  Invalid
}

