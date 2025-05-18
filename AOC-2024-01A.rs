use std::io;
use std::io::BufRead;
use std::collections::BinaryHeap;

// We can find the difference between the lists by reading in the pairs
// of numbers into two separate priority heaps, then popping through
// both heaps, finding the difference between the elements. Add that
// difference to a running total.

fn main() {
  // Our two heaps.
  let mut lefts: BinaryHeap<i32> = BinaryHeap::new();
  let mut rights: BinaryHeap<i32> = BinaryHeap::new();

  // Read all the lines.
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    // The line should be made of two numbers separated by whitespace.
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() == 2);
    // Collect the values into our heaps.
    let left = parts[0].parse::<i32>().unwrap();
    lefts.push(left);
    let right = parts[1].parse::<i32>().unwrap();
    rights.push(right);
  }

  // Keep a running total of the differences.
  let mut ttl = 0;
  while let Some(left) = lefts.pop() {
    let right = rights.pop().unwrap();
    ttl += (left - right).abs();
  }

  println!("{}", ttl);
}
