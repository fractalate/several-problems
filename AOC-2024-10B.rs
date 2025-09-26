use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::BufRead;

struct Map {
  data: Vec<Vec<i64>>,
  width: usize,
  height: usize,
}

fn read_value(c: u8) -> Option<i64> {
  if c >= b'0' && c <= b'9' {
    return Some((c - b'0') as i64);
  }
  return None;
}

fn read_map() -> Map {
  let mut data: Vec<Vec<i64>> = Vec::new();

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(Ok(line)) = lines.next() {
    let mut data_line: Vec<i64> = Vec::new();

    for c in line.bytes() {
      let value = read_value(c).unwrap_or(-2);
      data_line.push(value);
    }

    if !data.is_empty() {
      assert!(data_line.len() == data[0].len());
    }

    data.push(data_line);
  }

  assert!(!data.is_empty());

  let width = data[0].len();
  let height = data.len();

  return Map{
    data,
    width,
    height,
  };
}

fn calculate_scores(map: Map) -> i64 {
  let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
  let mut visited: HashSet<(usize, usize)> = HashSet::new();
  let mut reachable_nines: HashMap<(usize, usize), i64> = HashMap::new();
  let mut total_score = 0;

  for y in 0..map.height {
    for x in 0..map.width {
      if map.data[y][x] == 9 {
        reachable_nines.insert((x, y), 1);
        queue.push_back((x, y));
      }
    }
  }

  while let Some((x, y)) = queue.pop_front() {
    let mut check = |x2: usize, y2: usize| {
      if y2 < map.height && x2 < map.width {
        // Ensure the move is a step down.
        if map.data[y2][x2] == map.data[y][x] - 1 {
          // Any 9 reachable by the parent node is reachable by the child.
          if let Some(parent_nines) = reachable_nines.get(&(x, y)) {
            let new_nines = if let Some(nines) = reachable_nines.get(&(x2, y2)) {
              nines + parent_nines
            } else {
              *parent_nines
            };

            reachable_nines.insert((x2, y2), new_nines);
          }

          if visited.insert((x2, y2)) {
            queue.push_back((x2, y2));
          }
        }
      }
    };

    check(x, y.wrapping_sub(1));
    check(x, y+1);
    check(x.wrapping_sub(1), y);
    check(x+1, y);

    // Sum the number of ways to get to each 9.
    if let Some(nines) = reachable_nines.remove(&(x, y)) {
      if map.data[y][x] == 0 {
        total_score += nines;
      }
    }
  }

  return total_score as i64;
}

fn main() {
  let map = read_map();
  let final_score = calculate_scores(map);
  println!("{final_score}");
}
