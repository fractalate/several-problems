use std::io;
use std::io::BufRead;
use std::ops::Div;

fn get_next_position(c: u8, x: usize, y: usize) -> Option<(usize, usize)> {
  let mut nx = x;
  let mut ny = y;

  if c == b'v' {
    ny += 1;
  } else if c == b'^' {
    ny = ny.wrapping_sub(1);
  } else if c == b'<' {
    nx = nx.wrapping_sub(1);
  } else if c == b'>' {
    nx += 1;
  } else {
    return None;
  }

  return Some((nx, ny));
}

fn get_rot_90(c: u8) -> Option<u8> {
  if c == b'v' {
    return Some(b'<');
  } else if c == b'^' {
    return Some(b'>');
  } else if c == b'<' {
    return Some(b'^');
  } else if c == b'>' {
    return Some(b'v');
  }

  return None;
}

fn read_matrix() -> (Vec<Vec<u8>>, usize, usize, usize, usize) {
  let mut matrix: Vec<Vec<u8>> = Vec::new();
  let mut x = 0;
  let mut y = 0;
  let mut x_bound = 0;
  let mut y_bound = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.reserve(line.len());
    x_bound = 0;
    for c in line.bytes() {
      if c == b'v' || c == b'^' || c == b'<' || c == b'>' {
        x = x_bound;
        y = y_bound;
      }
      bytes.push(c);
      x_bound += 1;
    }
    matrix.push(bytes);
    y_bound += 1;
  }

  return (matrix, x, y, x_bound, y_bound);
}

enum SimulationResult {
  OutOfBounds,
  Loop,
}

fn run_simulation(mut matrix: Vec<Vec<u8>>, mut x: usize, mut y: usize, x_bound: usize, y_bound: usize) -> SimulationResult {
  let mut count = 1;
  let mut count_distinct = 1;

  loop {
    let c = matrix[y][x];
    let (nx, ny) = get_next_position(c, x, y).unwrap();

    if ny >= y_bound || nx >= x_bound { // Encompasses wrap around from 0 -> usize::MAX.
      return SimulationResult::OutOfBounds;
    }

    let nc = matrix[ny][nx];

    if nc == b'#' {
      matrix[y][x] = get_rot_90(c).unwrap();
    } else if nc == b'.' || nc == b'X' {
      matrix[y][x] = b'X';
      matrix[ny][nx] = c;

      if nc != b'X' {
        count_distinct += 1;
      }
      count += 1;

      // If they haven't found a new space in two times around, then they're in a cycle.
      if count_distinct < count.div(2) {
        return SimulationResult::Loop;
      }

      x = nx;
      y = ny;
    }
  }
}

fn main() {
  let (matrix, x, y, x_bound, y_bound) = read_matrix();

  let mut count_loops = 0;

  for iy in 0..y_bound {
    for ix in 0..x_bound {
      if matrix[iy][ix] == b'.' {
        let mut matrix2: Vec<Vec<u8>> = matrix.iter().map(|row| row.clone()).collect();
        matrix2[iy][ix] = b'#';
        match run_simulation(matrix2, x, y, x_bound, y_bound) {
          SimulationResult::Loop => {
            count_loops += 1;
          },
          _ => {},
        }
      }
    }
  }

  println!("{count_loops}");
}
