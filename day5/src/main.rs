use std::fs;

#[derive(Debug, Clone)]
struct Seat {
  row: u8,
  column: u8,
  id: u32
}

impl Seat {
  fn new(row: u8, column: u8) -> Seat {
    Seat { row: row, column: column, id: (row as u32) * 8 + column as u32 }
  }
}

impl std::cmp::PartialEq for Seat {
  fn ne(&self, other: &Self) -> bool {
    self.id != other.id
  }

  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

fn main() {
  let contents = fs::read_to_string("./input.txt")
    .expect("Cannot load file.");

  let numbers: Vec<&str> = contents.split("\n").collect();

  let seat_ids: Vec<u32> =
    numbers
      .iter()
      .map(|number| build_seat(number).id)
      .collect();

  let missing_seats = get_missing_seats(&seat_ids);
  println!("Missing seats ({}):", missing_seats.len());
  for seat in missing_seats.iter() {
    println!("{:?}", seat);
  }
}

fn get_missing_seats(seat_ids: &Vec<u32>) -> Vec<Seat> {
  let mut all_seats: Vec<Seat> = Vec::new();
  for row in 1..126 {
    for col in 0..7 {
      all_seats.push( Seat::new(row, col) );
    }
  }

  println!("All: {}", all_seats.len());

  let mut missing_seats: Vec<Seat> =
    all_seats
      .iter()
      .cloned()
      .filter(|seat| !seat_ids.contains(&seat.id))
      .filter(|seat| contains_neighbor(seat_ids, seat.id))
      .collect();

  missing_seats.sort_by(|a, b| a.id.cmp(&b.id));
  missing_seats
}

fn contains_neighbor(seat_ids: &Vec<u32>, seat_id: u32) -> bool {
  for id in seat_ids {
    let diff: i32 = 0 + (*id as i32) - (seat_id as i32);

    if diff.abs() == 1 {
      return true;
    }
  }
  false
}

fn build_seat(seat_number: &str) -> Seat {
  let nbrs: Vec<char> = seat_number.chars().collect();

  let (row, col) = evaluate_seat_number(nbrs, 0, (0, 127), (0, 7));
  Seat::new(row, col)
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
  let middle = (((highest - lowest) as f64) / 2.0).floor() as u8;
  (lowest, lowest + middle)
}

fn take_upper_half(lowest: u8, highest: u8) -> (u8, u8) {
  let middle = (((highest - lowest) as f64) / 2.0).ceil() as u8;
  (lowest + middle, highest)
}
