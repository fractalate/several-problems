// Project Euler 22

use std::{
  fs::File,
  io::{self, Read},
};

fn read_names_txt() -> io::Result<Vec<String>> {
  let mut file = File::open("files/PE-22.names.txt")?;
  let mut contents = String::new();

  file.read_to_string(&mut contents)?;

  let names = contents
    .split(',')
    .map(|quoted_name| String::from(&quoted_name[1..quoted_name.len()-1]))
    .collect();

  Ok(names)
}

fn calculate_name_score(name: &str) -> u64 {
  let mut score: u64 = 0;

  for c in name.chars() {
    score += (c  as u64) - ('A' as u64) + 1;
  }

  score
}

fn find_total_name_score() -> io::Result<u64> {
  let mut names = read_names_txt()?;
  names.sort();

  let mut total_score = 0;

  for i in 0..names.len() {
    let name_number = (i as u64) + 1;
    total_score += name_number * calculate_name_score(&names[i].as_str());
  }

  Ok(total_score)
}

fn main() -> io::Result<()> {
  println!("{}", find_total_name_score()?);

  Ok(())
}
