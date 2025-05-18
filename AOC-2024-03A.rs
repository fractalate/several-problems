use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut ttl = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    
    for (_, [arg1, arg2]) in re.captures_iter(&line).map(|c| c.extract()) {
      let value1 = arg1.parse::<u64>().unwrap();
      let value2 = arg2.parse::<u64>().unwrap();
      ttl += value1 * value2;
    }
  }

  println!("{}", ttl);
}
