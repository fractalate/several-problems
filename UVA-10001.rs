// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=942

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

    let rule_number = parts[0].parse::<u8>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid integer"))?;
    let number_of_cells = parts[1].parse::<usize>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid integer"))?;
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
      return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid problem format"));
    }

    // TODO - return something good
    return Ok(Some(Problem {
      rule_number,
      target_state,
    }));

  }

  return Ok(None);
}

enum Solution {
  GardenOfEden,
  Reachable,
}

fn apply_rule(rule_number: u8, left: u8, cell: u8, right: u8) -> u8 {
  let which_case = (left << 2) | (cell << 1) | right;
  return (rule_number >> which_case) & 1;
}

fn shit2(problem: &Problem) -> Solution {
  let target_value = problem.target_state[0];
  for i in 0..8 as u8 {
    let left = (i >> 2) & 1;
    let cell = (i >> 1) & 1;
    let right = i & 1;

    let value = apply_rule(problem.rule_number, left, cell, right);

    if value == target_value {
      if let Solution::Reachable = shit(problem, 1, left, cell, cell, right) {
        return Solution::Reachable;
      }
    }
  }

  return Solution::GardenOfEden;
}

fn shit(problem: &Problem, position: usize, tail: u8, first: u8, prev_cell: u8, prev_right: u8) -> Solution {
  let target_value = problem.target_state[position];
  for i in 0..8 as u8 {
    let left = (i >> 2) & 1;
    let cell = (i >> 1) & 1;
    let right = i & 1;

    if left != prev_cell || cell != prev_right {
      continue
    } else if position + 2 == problem.target_state.len() {
      if right != tail {
        continue;
      }
    } else if position + 1 == problem.target_state.len() {
      if right != first {
        continue;
      }
    }

    let value = apply_rule(problem.rule_number, left, cell, right);

    if value == target_value {
      if position + 1 == problem.target_state.len() {
        return Solution::Reachable;
      } else {
        if let Solution::Reachable = shit(problem, position + 1, tail, first, cell, right) {
          return Solution::Reachable;
        }
      }
    }
  }

  return Solution::GardenOfEden;
}

fn solve_problem(problem: &Problem) -> Solution {
  return shit2(problem);
}

fn main() -> Result<()> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(problem) = read_problem(&mut lines)? {
    match solve_problem(&problem) {
      Solution::GardenOfEden => println!("GARDEN OF EDEN"),
      Solution::Reachable => println!("REACHABLE"),
    }
  }

  Ok(())
}
