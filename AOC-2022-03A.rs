use std::io;
use std::io::BufRead;
use std::ops::Div;
use std::collections::HashSet;

fn letter_priority(me: char) -> u32 {
  if me >= 'a' && me <= 'z' {
    return u32::from(me) - u32::from('a') + 1;
  } else {
    return u32::from(me) - u32::from('A') + 1 + 26;
  }
}

fn main() {
  let mut total: u32 = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let line = line.to_string();
    let half = line.len().div(2);
    let mut left: HashSet<char> = HashSet::new();
    let lstr = &line[..half];
    for c in lstr.chars() {
      left.insert(c);
    }
    let rstr = &line[half..];
    let mut right: HashSet<char> = HashSet::new();
    for c in rstr.chars() {
      right.insert(c);
    }
    let common = left.intersection(&right);
    for c in common {
      total += letter_priority(*c);
    }
  }
  println!("{}", total);
}
