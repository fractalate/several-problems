// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=942
//
// This problem asks us to determine whether a given 1D cellular automaton state is reachable
// from some previous state.

use std::io::{
  self,
  BufRead,
  Lines,
  Result,
  StdinLock,
};

struct Problem {
  rule_number: u8,
  target_state: Vec<u8>,
}

fn read_problem(lines: &mut Lines<StdinLock<'static>>) -> Result<Option<Problem>> {
  if let Some(data) = lines.next() {
    let line = data?;
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 3 {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid problem format"));
    }

    let rule_number = parts[0].parse::<u8>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid rule number"))?;
    let number_of_cells = parts[1].parse::<usize>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid number of cells"))?;
    let mut target_state: Vec<u8> = Vec::new();
    target_state.reserve(number_of_cells);

    for c in parts[2].chars() {
      if c == '0' {
        target_state.push(0);
      } else if c == '1' {
        target_state.push(1);
      } else {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid target state"));
      }
    }

    if number_of_cells != target_state.len() {
      return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid target state length"));
    }

    return Ok(Some(Problem {
      rule_number,
      target_state,
    }));
  }

  return Ok(None);
}

fn apply_rule(rule_number: u8, left: u8, cell: u8, right: u8) -> u8 {
  let which_case = (left << 2) | (cell << 1) | right;
  return (rule_number >> which_case) & 1;
}

enum Solution {
  GardenOfEden,
  Reachable,
}

fn solve_problem(problem: &Problem, position: usize, first: u8, tail: u8, prev_cell: u8, prev_right: u8) -> Solution {
  let target_value = problem.target_state[position];

  for i in 0..8 as u8 {
    let left = (i >> 2) & 1;
    let cell = (i >> 1) & 1;
    let right = i & 1;

    if position > 0 && (left != prev_cell || cell != prev_right) {
      continue
    } else if position + 2 == problem.target_state.len() && right != tail {
      continue;
    } else if position + 1 == problem.target_state.len() && right != first {
      continue;
    }

    let value = apply_rule(problem.rule_number, left, cell, right);

    if value == target_value {
      if position + 1 == problem.target_state.len() {
        return Solution::Reachable;
      }
      
      let tail = if position == 0 { left } else { tail };
      let first = if position == 0 { cell } else { first };

      if let Solution::Reachable = solve_problem(problem, position + 1, first, tail, cell, right) {
        return Solution::Reachable;
      }
    }
  }

  return Solution::GardenOfEden;
}

fn print_solution(problem: &Problem) {
  match solve_problem(problem, 0, 0, 0, 0, 0) {
    Solution::GardenOfEden => println!("GARDEN OF EDEN"),
    Solution::Reachable => println!("REACHABLE"),
  }
}

fn main() -> Result<()> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(problem) = read_problem(&mut lines)? {
    print_solution(&problem);
  }

  Ok(())
}
