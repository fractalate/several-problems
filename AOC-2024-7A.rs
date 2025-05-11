use std::io;
use std::io::BufRead;

fn can_reach_target(target: u64, accumulated: u64, items: &[u64]) -> bool {
  if items.len() == 0 {
    return accumulated == target;
  } else if can_reach_target(target, accumulated + items[0], &items[1..]) {
    return true;
  } else if can_reach_target(target, accumulated * items[0], &items[1..]) {
    return true;
  }
  return false;
}

fn main() {
  let mut total = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let mut iter = line.split_whitespace();

    // Determine the target value.
    let target = iter.next().unwrap();
    let target = &target[0..target.len()-1];
    let target = target.parse::<u64>().unwrap();

    let mut items: Vec<u64> = Vec::new();
    while let Some(item) = iter.next() {
      items.push(item.parse::<u64>().unwrap());
    }
    assert!(items.len() > 0);

    if can_reach_target(target, items[0], &items[1..]) {
      total += target;
    }
  }

  println!("{total}");
}
