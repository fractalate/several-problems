use std::io;
use std::io::BufRead;

fn xmas(x: u8, m: u8, a: u8, s: u8) -> bool {
  return x == b'X' && m == b'M' && a == b'A' && s == b'S';
}

fn main() {
  let mut matrix: Vec<Vec<u8>> = Vec::new();

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut w: usize = 0;
  while let Some(Ok(line)) = lines.next() {
    w = line.len(); // assume the width is the same for each line
    matrix.push(line.bytes().collect());
  }

  let mut count = 0;

  for r in 0..matrix.len() {
    for c in 0..w {

      // To the right.
      if c + 3 < w {
        if xmas(matrix[r][c], matrix[r][c+1], matrix[r][c+2], matrix[r][c+3]) {
          count += 1;
        }
      }

      // To the right and down.
      if c + 3 < w && r + 3 < matrix.len() {
        if xmas(matrix[r][c], matrix[r+1][c+1], matrix[r+2][c+2], matrix[r+3][c+3]) {
          count += 1;
        }
      }

      // To the right and up.
      if c + 3 < w && r > 2 {
        if xmas(matrix[r][c], matrix[r-1][c+1], matrix[r-2][c+2], matrix[r-3][c+3]) {
          count += 1;
        }
      }

      // To the left.
      if c > 2 {
        if xmas(matrix[r][c], matrix[r][c-1], matrix[r][c-2], matrix[r][c-3]) {
          count += 1;
        }
      }

      // To the left and down.
      if c > 2 && r + 3 < matrix.len() {
        if xmas(matrix[r][c], matrix[r+1][c-1], matrix[r+2][c-2], matrix[r+3][c-3]) {
          count += 1;
        }
      }

      // To the left and up.
      if c > 2 && r > 2 {
        if xmas(matrix[r][c], matrix[r-1][c-1], matrix[r-2][c-2], matrix[r-3][c-3]) {
          count += 1;
        }
      }

      // Down.
      if r + 3 < matrix.len() {
        if xmas(matrix[r][c], matrix[r+1][c], matrix[r+2][c], matrix[r+3][c]) {
          count += 1;
        }
      }

      // Up.
      if r > 2 {
        if xmas(matrix[r][c], matrix[r-1][c], matrix[r-2][c], matrix[r-3][c]) {
          count += 1;
        }
      }


    }
  }

  println!("{}", count);
}
