// cargo-deps: itertools = "0.13.0"
// rust-script ICPC-NAC2021A.rs < ./files/ICPC-NAC2021A.input

// ICPC - North American Contest, 2023, Problem A

// https://icpcarchive.github.io/North%20America%20Contests/North%20America%20Championship/2021%20North%20America%20Championship/problems.pdf

// For the given sequence, it is guaranteed that for all nonnegative numbers x, if there is some i such
// that ai & x = x, then there is a j such that aj = x. Here, & refers to the bitwise AND operator.

// contrapositive: if aj != x for all j, then ai & x != x for all i.

use itertools::Itertools;
use std::io;

fn check_solution(a: &Vec<i64>, b: &Vec<i64>) -> bool {
  return a.iter().zip(b.iter()).all(|(x, y)| x & y == 0);
}

fn solution_brute_force(a: &Vec<i64>) -> Option<Vec<i64>> {
  for b in a.iter().cloned().permutations(a.len()) {
    if check_solution(a, &b) {
      return Some(b.iter().cloned().collect());
    }
  }
  return None;
}

fn solution(a: &Vec<i64>) -> Option<Vec<i64>> {
  return solution_brute_force(a); // lol
}

fn main() {
  let mut size_str = String::new();
  io::stdin().read_line(&mut size_str).expect("missing input");
  let size: usize = size_str.trim().parse().expect("invalid size");

  let mut a: Vec<i64> = Vec::with_capacity(size);

  for _ in 0..size {
    let mut value_str = String::new();
    io::stdin().read_line(&mut value_str).expect("missing input");
    let value: i64 = value_str.trim().parse().expect("invalid integer");

    a.push(value);
  }

  let solution = solution(&a).expect("solution not found!");
  assert!(check_solution(&a, &solution));
  println!("{:?}", solution);
}
