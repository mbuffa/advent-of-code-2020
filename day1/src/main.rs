use std::fs;

fn main() {
  let mut answer: u32 = 0;

  let contents = fs::read_to_string(r#"./input.txt"#)
    .expect("Cannot load file.");

  let expenses: Vec<&str> = contents.split("\n").collect();

  'outer: for a in expenses.iter() {
    let a_int: u32 = parse_int(a);

    for b in expenses.iter() {
      let b_int: u32 = parse_int(b);

      'inner: for c in expenses.iter() {
        if a == b && b == c {
          continue 'inner;
        }

        let c_int: u32 = parse_int(c);

        if a_int + b_int + c_int == 2020 && a_int > 0 && b_int > 0 && c_int > 0 {
          // println!("Found {} {} {}", a_int, b_int, c_int);
          answer = a_int * b_int * c_int;
          break 'outer;
        }
      }
    }
  }

  println!("{}", answer);
}

fn parse_int(number: &str) -> u32 {
  match number.parse::<u32>() {
    Ok(i) => i,
    Err(_) => 0
  }
}