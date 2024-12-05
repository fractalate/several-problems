use regex::Regex;
use std::io;
use std::io::BufRead;

fn main() {
  // Matches:
  // * mul(\d+,\d+) matches to produce tuple ("mul(\d+,\d+)", "\d+", "\d+")
  // * do() matches to produce tuple ("do()", "(", ")")
  // * don't() matches to produce tuple ("don't()", "(", ")")
  let re = Regex::new(r"(mul\((\d+),(\d+)\)|do(\()(\))|don't(\()(\)))").unwrap();
  let mut ttl = 0;
  let mut active = true;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    
    let mut results = vec![];
    for (_, [operator, arg1, arg2]) in re.captures_iter(&line).map(|c| c.extract()) {
        results.push((operator, arg1, arg2));
    }

    for (operator, arg1, arg2) in results {
      if operator.starts_with("mul") {
        if active {
          ttl += arg1.parse::<u64>().unwrap() * arg2.parse::<u64>().unwrap();
        }
      } else if operator == "do()" {
        active = true;
      } else if operator == "don't()" {
        active = false;
      }
    }
  }

  println!("{}", ttl);
}
