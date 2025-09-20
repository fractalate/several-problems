use std::io;
use std::io::BufRead;

fn main() {
  let mut total: i64 = 0;
  let mut maximum: i64 = 0;
  let mut done = false;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while !done {
    let mut line = "".to_string();
    if let Some(Ok(this_line)) = lines.next() {
      line = this_line;
    } else {
      done = true;
    }
    if line.len() > 0 {
      let v = line.parse::<i64>().unwrap();
      total += v;
    } else {
      maximum = maximum.max(total);
      total = 0;
    }
  }

  println!("{}", maximum);
}
