use std::io;
use std::io::BufRead;
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
  let mut group: i64 = 0;
  let mut common_common: HashSet<char> = HashSet::new();
  while let Some(Ok(line)) = lines.next() {
    let line = line.to_string();
    let common: HashSet<char> = line[..].chars().into_iter().collect();

    group += 1;

    if group == 1 {
      common_common = common.clone();
    } else if group <= 2 {
      common_common = common.intersection(&common_common).map(|x| *x).collect();
    } else {
      common_common = common.intersection(&common_common).map(|x| *x).collect();
      for c in common_common.clone() {
        total += letter_priority(c).unwrap();
      }
      common_common.clear();
      group = 0;
    }
  }
  for c in common_common.clone() {
    total += letter_priority(c).unwrap();
  }
  println!("{}", total);
}
