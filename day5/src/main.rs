use std::fs;

struct Seat {
  row: u8,
  column: u8,
  id: u32
}

fn main() {
  let contents = fs::read_to_string("./input.txt")
    .expect("Cannot load file.");

  let numbers: Vec<&str> = contents.split("\n").collect();

  let highest_id =
    numbers
      .iter()
      .map(|number| find_seat(number))
      .max_by(|seat, other| seat.id.cmp(&other.id));

  println!("Seat with highest ID: {}", highest_id.unwrap().id);

  // let seat = find_seat("BFFFBBFRRR");
  // println!("{} {} {}", seat.row, seat.column, seat.id);
  // let seat = find_seat("FFFBBBFRRR");
  // println!("{} {} {}", seat.row, seat.column, seat.id);
  // let seat = find_seat("BBFFBBFRLL");
  // println!("{} {} {}", seat.row, seat.column, seat.id);
}

fn find_seat(seat_number: &str) -> Seat {
  let nbrs: Vec<char> = seat_number.chars().collect();

  let (row, col) = evaluate_seat_number(nbrs, 0, (0, 127), (0, 7));
  // println!("Built seat with {} {}", row, col);
  Seat { row: row, column: col, id: (row as u32) * 8 + col as u32 }
}

fn evaluate_seat_number(nbrs: Vec<char>, idx: usize, rows: (u8, u8), cols: (u8, u8)) -> (u8, u8) {
  if idx + 1 > nbrs.len() {
    return (rows.0, cols.0)
  }

  if nbrs[idx] == 'F' {
    evaluate_seat_number(nbrs, idx + 1, take_lower_half(rows.0, rows.1), cols)
  } else if nbrs[idx] == 'B' {
    evaluate_seat_number(nbrs, idx + 1, take_upper_half(rows.0, rows.1), cols)
  } else if nbrs[idx] == 'L' {
    evaluate_seat_number(nbrs, idx + 1, rows, take_lower_half(cols.0, cols.1))
  } else if nbrs[idx] == 'R' {
    evaluate_seat_number(nbrs, idx + 1, rows, take_upper_half(cols.0, cols.1))
  } else {
    panic!("Unrecognized character {}", nbrs[idx]);
  }
}

fn take_lower_half(lowest: u8, highest: u8) -> (u8, u8) {
  // println!("lower: {} {}", lowest, highest);
  let middle = (((highest - lowest) as f64) / 2.0).floor() as u8;
  (lowest, lowest + middle)
}

fn take_upper_half(lowest: u8, highest: u8) -> (u8, u8) {
  // println!("upper: {} {}", lowest, highest);
  let middle = (((highest - lowest) as f64) / 2.0).ceil() as u8;
  (lowest + middle, highest)
}
