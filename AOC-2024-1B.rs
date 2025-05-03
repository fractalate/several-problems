use std::io;
use std::io::BufRead;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// We can find the similarity score between the two lists by carefully popping
// elements from left and right heaps.

fn main() {
  // Our two heaps.
  let mut lefts: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
  let mut rights: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

  // Read all the lines.
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    // The line should be made of two numbers separated by whitespace.
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() == 2);
    // Collect the values into our heaps.
    let left = parts[0].parse::<i32>().unwrap();
    lefts.push(Reverse(left));
    let right = parts[1].parse::<i32>().unwrap();
    rights.push(Reverse(right));
  }

  let mut ttl = 0;

  // Run through the left heap.
  while let Some(Reverse(lv)) = lefts.pop() {
    let mut delta = 0; // How much this value on the left is worth after considering multiplicities on the right.

    // Skip until we find "lv" or run out of numbers.
    while let Some(Reverse(rv)) = rights.peek() {
      if *rv >= lv {
        break;
      }
      rights.pop();
    }

    // Add copies of "lv" for each copy of "lv" in the right heap (we're multiplying "lv" by the count here).
    while let Some(Reverse(rv)) = rights.peek() {
      if *rv != lv {
        break;
      }
      delta += lv;
      rights.pop();
    }

    // The total increases by that amount.
    ttl += delta;

    // Before we forget the calculation of delta, use it for all subsequent copies of "lv" on the left.
    while let Some(Reverse(lv2)) = lefts.peek() {
      if *lv2 != lv {
        break;
      }
      ttl += delta;
      lefts.pop();
    }
  }
  
  println!("{}", ttl);
}
