use std::io;
use std::io::BufRead;
use std::ops::Div;
use std::collections::HashSet;

fn letter_priority(me: char) -> Option<u32> {
  if me >= 'a' && me <= 'z' {
    return Some(u32::from(me) - u32::from('a') + 1);
  } else if me >= 'A' && me <= 'Z' {
    return Some(u32::from(me) - u32::from('A') + 1 + 26);
  }
  return None;
}

fn main() {
  let mut total: u32 = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let line = line.to_string();
    let half = line.len().div(2);
    let left: HashSet<char> = line[..half].chars().into_iter().collect();
    let right: HashSet<char> = line[half..].chars().into_iter().collect();
    
    let common = left.intersection(&right);
    for c in common {
      total += letter_priority(*c).unwrap();
    }
  }
  println!("{}", total);
}
