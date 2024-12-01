use std::io;
use std::io::BufRead;

// We can find the difference between the lists by reading in the pairs
// of numbers into two separate lists, sorting each list, then iterating
// through both lists finding the difference between the elements. Add
// that difference to a running total.

fn main() {
  // Our two lists.
  let mut lefts: Vec<i32> = Vec::new();
  let mut rights: Vec<i32> = Vec::new();

  // Read all the lines.
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    // The line should be made of two numbers separated by whitespace.
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() == 2);
    // Collect the values into our lists.
    let left = parts[0].parse::<i32>().unwrap();
    lefts.push(left);
    let right = parts[1].parse::<i32>().unwrap();
    rights.push(right);
  }

  // Sort so that the smallest and so on between the two lists are in the same positions
  // in their respective lists.
  lefts.sort();
  rights.sort();

  // Keep a running total of the differences.
  let mut ttl = 0;
  for i in 0..lefts.len() {
    ttl += (lefts[i] - rights[i]).abs();
  }

  println!("{}", ttl);
}
