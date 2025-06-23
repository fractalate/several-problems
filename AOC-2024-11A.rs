use std::io;
use std::io::BufRead;

fn blink(stone: u64, count_blinks: usize) -> usize {
  if count_blinks == 0 {
    return 1;
  } else if stone == 0 {
    return blink(1, count_blinks - 1);
  }

  let text = stone.to_string();
  if text.len() % 2 == 0 {
    let (left, right) = text.split_at(text.len() / 2);
    let left = left.parse::<u64>().unwrap();
    let right = right.parse::<u64>().unwrap();
    return blink(left, count_blinks - 1) + blink(right, count_blinks - 1);
  }

  return blink(stone * 2024, count_blinks - 1);
}

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(Ok(line)) = lines.next() {
    let iter = line.split_whitespace();
    let stones: Vec<u64> = iter.map(|x| x.parse::<u64>().unwrap()).collect();
    let mut total = 0;
    for stone in stones.iter() {
      total += blink(*stone, 25);
    }
    println!("{total}");
  }
}
