use std::io;
use std::io::BufRead;
use std::collections::BinaryHeap;

const COUNT_TOPS: usize = 3;

fn main() {
  let mut total: i64 = 0;
  let mut top: BinaryHeap<i64> = BinaryHeap::new();
  let mut done = false;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while !done {
    let mut line = "".to_string();
    if let Some(Ok(this_line)) = lines.next() {
      line = this_line;
    } else {
      done = true;
    }
    if line.len() > 0 {
      let v = line.parse::<i64>().unwrap();
      total += v;
    } else {
      top.push(-total); // negation for max heap
      if top.len() > COUNT_TOPS {
        top.pop();
      }
      total = 0;
    }
  }

  let maximum = -top.iter().sum::<i64>(); // negation for max heap

  println!("{}", maximum);
}
