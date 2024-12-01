use std::io;
use std::io::BufRead;

// We can find the similarity score between the two lists by carefully iterating
// through both lists after sorting. 

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

  let mut ttl = 0;
  let mut l = 0;
  let mut r = 0;

  // Run through the left list.
  while l < lefts.len() {
    let lv = lefts[l]; // Value on the left: "left value" or "lv".
    let mut delta = 0; // How much this value on the left is worth after considering multiplicities on the right.

    // Skip until we find "lv" or run out of numbers.
    while r < rights.len() && rights[r] < lv {
      r += 1;
    }
    // Add copies of "lv" for each copy of "lv" in the right list (we're multiplying "lv" by the count here).
    while r < rights.len() && rights[r] == lv {
      delta += lv;
      r += 1;
    }

    // The total increases by that amount.
    ttl += delta;
    l += 1;

    // Before we forget the calculation of delta, use it for all subsequent copies of "lv" on the left.
    while l < lefts.len() && lv == lefts[l] {
      ttl += delta;
      l += 1;
    }
  }

  println!("{}", ttl);
}
