use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut ttl = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    
    let mut results = vec![];
    for (_, [arg1, arg2]) in re.captures_iter(&line).map(|c| c.extract()) {
        results.push((arg1.parse::<u64>().unwrap(), arg2.parse::<u64>().unwrap()));
    }

    for (arg1, arg2) in results {
      ttl += arg1 * arg2;
    }
  }

  println!("{}", ttl);
}
