use std::collections::HashMap;
use std::io;
use std::io::BufRead;

struct Blinker {
  memo: HashMap<(u64, usize), usize>,
}

impl Blinker {
  fn new() -> Self {
    return Blinker{
      memo: HashMap::new(),
    }
  }

  fn blink(&mut self, stone: u64, count_blinks: usize) -> usize {
    let count = self.memo.get(&(stone, count_blinks));
    if let Some(result) = count {
      return *result;
    }
    let result = self.think_blink(stone, count_blinks);
    self.memo.insert((stone, count_blinks), result);
    return result;
  }

  fn think_blink(&mut self, stone: u64, count_blinks: usize) -> usize {
    if count_blinks == 0 {
      return 1;
    } else if stone == 0 {
      return self.blink(1, count_blinks - 1);
    }
  
    let text = stone.to_string();
    if text.len() % 2 == 0 {
      let (left, right) = text.split_at(text.len() / 2);
      let left = left.parse::<u64>().unwrap();
      let right = right.parse::<u64>().unwrap();
      let result = self.blink(left, count_blinks - 1);
      return result + self.blink(right, count_blinks - 1);
    }
  
    return self.blink(stone * 2024, count_blinks - 1);
  }
}

fn main() {
  let mut blinker = Blinker::new();

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(Ok(line)) = lines.next() {
    let iter = line.split_whitespace();
    let stones: Vec<u64> = iter.map(|x| x.parse::<u64>().unwrap()).collect();
    let mut total = 0;
    for stone in stones.iter() {
      total += blinker.blink(*stone, 75);
    }
    println!("{total}");
  }
}
