use std::io;
use std::io::BufRead;

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

  for r in 1..matrix.len()-1 {
    for c in 1..w-1 {
      if matrix[r][c] == b'A' {
        let ul = matrix[r-1][c-1];
        let ur = matrix[r-1][c+1];
        let ll = matrix[r+1][c-1];
        let lr = matrix[r+1][c+1];

        if ul == b'M' && ur == b'M' && lr == b'S' && ll == b'S' {
          count += 1;
        } else if ul == b'S' && ur == b'M' && lr == b'M' && ll == b'S' {
          count += 1;
        } else if ul == b'S' && ur == b'S' && lr == b'M' && ll == b'M' {
          count += 1;
        } else if ul == b'M' && ur == b'S' && lr == b'S' && ll == b'M' {
          count += 1;
        }
      }
    }
  }

  println!("{}", count);
}
