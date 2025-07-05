// Project Euler 1

fn multiples_of_3_or_5(below_this_number: u64) -> u64 {
  let mut sum = 0;

  for n in 1..below_this_number {
    if n % 5 == 0 || n % 3 == 0 {
      sum += n;
    }
  }

  sum
}

fn main() {
  println!("{}", multiples_of_3_or_5(10));
  println!("{}", multiples_of_3_or_5(1000));
}
