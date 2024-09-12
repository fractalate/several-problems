// UVa Online Judge 100

use std::io;

fn count_collatz_steps(number: i32) -> i64 {
  if number <= 1 {
    return 1;
  }

  let mut n = number;
  let mut steps = 1;

  while n != 1 {
    if n % 2 == 0 {
      n /= 2;
    } else {
      n = 3*n + 1;
    }
    steps += 1;
  }

  steps
}

fn max_collatz_steps_in_range(start: i32, end: i32) -> i64 {
  let mut most_steps = count_collatz_steps(start);
  let mut index = end;

  while index > start {
    most_steps = most_steps.max(count_collatz_steps(index));
    index -= 1;
  }

  most_steps
}

fn main() {
  let mut line = String::new();

  loop {
    line.clear();

    let bytes_read = io::stdin().read_line(&mut line)
      .expect("Failed to read line");

    if bytes_read == 0 {
      break;
    }

    let mut parts = line.split_whitespace();

    let start: i32 = parts.next()
      .expect("Missing start of range.")
      .parse()
      .expect("Invalid number specified for start of range.");

    let end: i32 = parts.next()
      .expect("Missing end of range.")
      .parse()
      .expect("Invalid number specified for end of range.");

    println!("{} {} {}", start, end, max_collatz_steps_in_range(start, end));
  }
}
