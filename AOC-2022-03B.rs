use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn letter_priority(me: char) -> Option<u64> {
  if me >= 'a' && me <= 'z' {
    return Some(u64::from(me) - u64::from('a') + 1);
  } else if me >= 'A' && me <= 'Z' {
    return Some(u64::from(me) - u64::from('A') + 1 + 26);
  }
  return None;
}

fn main() {
  let mut total: u64 = 0;
  let mut common: HashSet<char> = HashSet::new();

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut group: i64 = 0;
  while let Some(Ok(line)) = lines.next() {
    let chars: HashSet<char> = line.chars().into_iter().collect();

    group += 1;

    if group == 1 {
      common = chars.clone();
    } else {
      common = chars.intersection(&common).map(|x| *x).collect();

      if group >= 3 {
        for c in &common {
          total += letter_priority(*c).unwrap();
        }
        common.clear();
        group = 0;
      }
    }
  }

  println!("{}", total);
}
