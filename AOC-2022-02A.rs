use std::io;
use std::io::BufRead;

fn score_for_my_choice(me: &str) -> i64 {
  if me == "X" {
    return 1;
  } else if me == "Y" {
    return 2;
  } else if me == "Z" {
    return 3;
  }
  return 0;
}

fn score(opponent: &str, me: &str) -> i64 {
  let mut result: i64 = 0;

  if opponent == "A" && me == "X" {
    result += 3;
  } else if opponent == "B" && me == "Y" {
    result += 3;
  } else if opponent == "C" && me == "Z" {
    result += 3;
  } else if opponent == "A" && me == "Y" {
    result += 6;
  } else if opponent == "B" && me == "Z" {
    result += 6;
  } else if opponent == "C" && me == "X" {
    result += 6;
  }

  result += score_for_my_choice(me);

  return result;
}

fn main() {
  let mut total: i64 = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() == 2);
    let s = score(parts[0], parts[1]);
    total += s;
  }
  println!("{}", total);
}
