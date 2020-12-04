use std::fs;

type Tiles = Vec<Vec<char>>;

struct Map {
  tiles: Tiles
}

impl Map {
  fn load(filename: &str) -> Map {
    let contents = fs::read_to_string(filename)
      .expect(format!("Cannot load {}", filename).as_str());

    let lines: Vec<&str> = contents.split("\n").collect();

    let rows_count = lines.len();
    let cols_count = lines[0].len();

    let mut tiles: Vec<Vec<char>> = vec![vec!['#'; cols_count]; rows_count];

    for (y, line) in lines.iter().enumerate() {
      let chars: Vec<char> = line.chars().collect();

      for (x, tile) in chars.iter().enumerate() {
        println!("{} {}", y, x);
        tiles[y][x] = tile.clone();
      }
    }

    Map {
      tiles: tiles
    }
  }

  fn do_move_by(&self,
    x: usize,
    y: usize,
    position: &mut [usize; 2],
    dimensions: [usize; 2],
    collisions: &mut u32) -> u32
  {
    position[0] += y;
    position[1] += x;

    if position[0] >= dimensions[0] {
      return collisions.clone();
    }

    if position[1] >= dimensions[1] {
      position[1] = position[1] - dimensions[1];
    }

    let probe_y = position[0];
    let probe_x = position[1];

    if self.tiles[probe_y][probe_x] == '#' {
      *collisions += 1;
    }

    self.do_move_by(x, y, position, dimensions, collisions)
  }

  fn move_by(&self, x: usize, y: usize) -> u32 {
    let tiles = &self.tiles;

    let mut position: [usize; 2] = [0, 0];
    let dimensions = [tiles.len(), tiles[0].len()];
    let mut collisions: u32 = 0;

    self.do_move_by(
      x, y,
      &mut position,
      dimensions,
      &mut collisions
    )
  }
}

fn main() {
  let map = Map::load("./input.txt");

  let mut collisions: Vec<u32> = Vec::new();
  collisions.push(map.move_by(1, 1));
  collisions.push(map.move_by(3, 1));
  collisions.push(map.move_by(5, 1));
  collisions.push(map.move_by(7, 1));
  collisions.push(map.move_by(1, 2));

  println!("Collisions: {}", collisions.iter().fold(1, |acc, c| acc * c));
}

