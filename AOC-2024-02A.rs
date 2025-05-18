use std::io;
use std::io::BufRead;

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

    // A report is safe if it is strictly trending AND it doesn't increase too much (is "under control").
    let mut last_last_number = parts[0];
    let mut last_number = parts[1];
    let mut trending = last_last_number != last_number;
    let mut under_control = (last_last_number - last_number).abs() <= 3;

    if trending && under_control {
      for i in 2..parts.len() {
        let number = parts[i];
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

    if trending && under_control {
      count_safe += 1;
    }
  }

  println!("{}", count_safe);
}
