use std::collections::{
  HashMap,
  HashSet,
};
use std::io;
use std::io::BufRead;

fn calculate_antipole_positions(x0: i32, y0: i32, x1: i32, y1: i32, width: i32, height: i32) -> Vec<(i32, i32)> {
  let mut result: Vec<(i32, i32)> = Vec::new();

  let dx = x1 - x0;
  let dy = y1 - y0;

  let mut x = x0;
  let mut y = y0;

  while x >= 0 && x < width && y >= 0 && y < height {
    result.push((x, y));
    x += dx;
    y += dy;
  }

  let mut x = x0 - dx;
  let mut y = y0 - dy;

  while x >= 0 && x < width && y >= 0 && y < height {
    result.push((x, y));
    x -= dx;
    y -= dy;
  }

  return result;
}

fn main() {
  let mut antenna_positions: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut height = 0;
  let mut width = 0;
  while let Some(Ok(line)) = lines.next() {
    width = 0;
    for c in line.bytes() {
      if c != b'.' {
        antenna_positions.entry(c).or_insert_with(Vec::new).push((width, height));
      }
      width += 1;
    }
    height += 1;
  }

  let mut antipoles = HashSet::new();
  for (_, positions) in antenna_positions.iter() {
    for i in 0..positions.len()-1 {
      for j in i+1..positions.len() {
        let (x0, y0) = positions[i];
        let (x1, y1) = positions[j];
        for antipole in calculate_antipole_positions(x0, y0, x1, y1, width, height) {
          antipoles.insert(antipole);
        }
      }
    }
  }

  let total = antipoles.len();
  println!("{total}");
}
