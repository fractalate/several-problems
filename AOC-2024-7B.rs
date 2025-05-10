use std::io;
use std::io::BufRead;

fn numcat(a: u64, b: u64) -> u64 {
  return format!("{a}{b}").parse::<u64>().unwrap();
}

fn can_reach_target(target: u64, items: &Vec<u64>) -> bool {
  if items.len() == 1 {
    return items[0] == target;
  } else {
    let mut rest = vec![items[0] + items[1]];
    rest.extend_from_slice(&items[2..]);
    if can_reach_target(target, &rest) {
      return true;
    }
    rest[0] = items[0] * items[1];
    if can_reach_target(target, &rest) {
      return true;
    }
    rest[0] = numcat(items[0], items[1]);
    if can_reach_target(target, &rest) {
      return true;
    }
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

    if can_reach_target(target, &items) {
      total += target;
    }
  }

  println!("{total}");
}
