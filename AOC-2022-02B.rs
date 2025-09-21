use std::io;
use std::io::BufRead;

const ROCK: i64 = 0;
const PAPER: i64 = 1;
const SCISSORS: i64 = 2;

const LOSE: i64 = 0;
const DRAW: i64 = 1;
const WIN: i64 = 2;

fn symbol_to_choice(symbol: &str) -> Option<i64> {
  if symbol == "A" {
    return Some(ROCK);
  } else if symbol == "B" {
    return Some(PAPER);
  } else if symbol == "C" {
    return Some(SCISSORS);
  }
  return None;
}

fn symbol_to_outcome(symbol: &str) -> Option<i64> {
  if symbol == "X" {
    return Some(LOSE);
  } else if symbol == "Y" {
    return Some(DRAW);
  } else if symbol == "Z" {
    return Some(WIN);
  }
  return None;
}

fn determine_choice_for_outcome(opponent_choice: i64, outcome: i64) -> i64 {
  if outcome == LOSE {
    return (opponent_choice + 2) % 3;
  } else if outcome == WIN {
    return (opponent_choice + 1) % 3;
  }
  opponent_choice
}

fn score_for_choice(choice: i64) -> i64 {
  return choice + 1; // ROCK 1, PAPER 2, SCISSORS 3
}

fn score_for_outcome(outcome: i64) -> i64 {
  return outcome * 3; // LOSE 0, DRAW 3, WIN 6
}

fn score(opponent_choice: i64, outcome: i64) -> i64 {
  let my_choice = determine_choice_for_outcome(opponent_choice, outcome);
  return score_for_outcome(outcome) + score_for_choice(my_choice);
}

fn main() {
  let mut total: i64 = 0;

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let choices: Vec<&str> = line.split_whitespace().collect();
    assert!(choices.len() == 2);

    total += score(
      symbol_to_choice(choices[0]).unwrap(),
      symbol_to_outcome(choices[1]).unwrap(),
    );
  }

  println!("{}", total);
}
