use std::collections::{
  HashMap,
  HashSet,
};
use std::io;
use std::io::BufRead;

fn calculate_antipole_positions(mut x0: usize, mut y0: usize, mut x1: usize, mut y1: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
  let mut result: Vec<(usize, usize)> = Vec::new();

  if x1 < x0 {
    (x0, x1) = (x1, x0);
    (y0, y1) = (y1, y0);
  }

  let dx = x1 - x0;

  let x2 = x0.wrapping_sub(dx);
  let y2 = if y1 < y0 { y0 + (y0 - y1) } else { y0.wrapping_sub(y1 - y0) };
  if x2 < width && y2 < height {
    result.push((x2, y2));
  }

  let x3 = x1 + dx;
  let y3 = if y1 < y0 { y1.wrapping_sub(y0 - y1) } else { y1 + (y1 - y0) };
  if x3 < width && y3 < height {
    result.push((x3, y3));
  }

  return result;
}

fn main() {
  let mut antenna_positions: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

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
