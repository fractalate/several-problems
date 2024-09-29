// Project Euler 21

fn sum_of_proper_divisors(number: u64) -> u64 {
  let mut total: u64 = 0;

  for divisor in 1..=number>>1 {
    if number % divisor == 0 {
      total += divisor;
    }
  }

  total
}

fn is_amicable_number(number: u64) -> bool {
  let pair = sum_of_proper_divisors(number);
  let anti_pair = sum_of_proper_divisors(pair);

  pair != number && anti_pair == number
}

fn sum_of_amicable_numbers_under(limit: u64) -> u64 {
  let mut total = 0;

  for i in 1..limit {
    if is_amicable_number(i) {
      total += i;
    }
  }

  total
}

fn main() {
  println!("{} {}", sum_of_proper_divisors(220), sum_of_proper_divisors(284));
  println!("{} {}", is_amicable_number(220), is_amicable_number(284));
  println!("Answer: {}", sum_of_amicable_numbers_under(10_000));
}
