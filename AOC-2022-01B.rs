use std::io;
use std::io::BufRead;

fn main() {
  let mut total: i64 = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut collection: Vec<i64> = Vec::new();
  while let Some(Ok(line)) = lines.next() { // TODO: combine the no-thing-found condition action with the post-loop action
    if line.len() == 0 {
      collection.push(total);
      total = 0;
    } else {
      let v = line.parse::<i64>().unwrap();
      total += v;
    }
  }
  collection.push(total);
  collection.sort();
  collection.reverse();

  let mut grand_total = 0;
  for i in 0..3 {
    grand_total += collection[i];
  }

  println!("{}", grand_total);
}
