use std::io;
use std::io::BufRead;

fn main() {
  let mut total: i64 = 0;
  let mut maximum: i64 = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() { // TODO: combine the no-thing-found condition action with the post-loop action
    if line.len() == 0 {
      if total > maximum {
        maximum = total;
      }
      total = 0;
    } else {
      let v = line.parse::<i64>().unwrap();
      total += v;
    }
  } 
  if total > maximum {
    maximum = total;
  }
  println!("{}", maximum);
}
