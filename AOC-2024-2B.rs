use std::io;
use std::io::BufRead;

fn check_safe(numbers: &Vec<i32>) -> bool {
  let mut last_last_number = numbers[0];
  let mut last_number = numbers[1];
  let mut trending = last_last_number != last_number;
  let mut under_control = (last_last_number - last_number).abs() <= 3;

  if trending && under_control {
    for i in 2..numbers.len() {
      let number = numbers[i];
      if last_last_number < last_number && last_number >= number {
        trending = false;
        break;
      } else if last_last_number > last_number && last_number <= number {
        trending = false;
        break;
      } else if (number - last_number).abs() > 3 { // 3 is the safety threshold.
        under_control = false;
        break;
      }
      last_last_number = last_number;
      last_number = number;
    }
  }

  return trending && under_control;
}

fn check_safe_damped(numbers: &Vec<i32>) -> bool {
  if check_safe(numbers) {
    return true;
  }

  for i in 0..numbers.len() {
    // Our inputs are pretty small, so cloning isn't the worst thing in the world.
    // An alternative approach would see an amended variation of check_safe which
    // operates on a sequence, instead of a randomly indexed array, and where the
    // variations of our sequence with the missing value are generated on the fly.
    let mut copies = numbers.clone();
    copies.remove(i);
    if check_safe(&copies) {
      return true;
    }
  }

  return false;
}

fn main() {
  // We'll count the safe reports.
  let mut count_safe = 0;

  // Read all the lines.
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    // Lines are list of numbers separated by whitespace.
    let parts: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    assert!(parts.len() > 1);

    if check_safe_damped(&parts) {
      count_safe += 1;
    }
  }

  println!("{}", count_safe);
}
