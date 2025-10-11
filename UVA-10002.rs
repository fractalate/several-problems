// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=943

use std::io::{self, Read, Result, StdinLock};

// Maybe some day we don't just read the whole input, but instead read portions and produce the words.
struct WordsIterator<'a> {
  stdin: &'a mut StdinLock<'static>,
  has_read: bool,
  words: Vec<String>,
  index: usize,
}

impl<'a> WordsIterator<'a> {
  fn new(stdin: &'a mut StdinLock<'static>) -> WordsIterator<'a> {
    WordsIterator {
      stdin,
      has_read: false,
      words: Vec::new(),
      index: 0,
    }
  }
}

impl<'a> Iterator for WordsIterator<'a> {
  type Item = Result<String>;

  fn next(&mut self) -> Option<Result<String>> {
    if !self.has_read {
      let mut buf: Vec<u8> = Vec::new();
      
      if let Err(e) = self.stdin.read_to_end(&mut buf) {
        return Some(Err(e));
      };

      let something = match String::from_utf8(buf) {
        Ok(something) => something,
        Err(_) => return Some(Err(io::Error::new(io::ErrorKind::InvalidData, "invalid utf8 input"))),
      };

      self.words = something.split_whitespace().map(String::from).collect();
      self.has_read = true;
    }

    if self.index < self.words.len() {
      let result = &self.words[self.index];
      self.index += 1;
      return Some(Ok(result.clone()));
    }

    return None;
  }
}

struct Problem {
  points: Vec<(f64, f64)>,
}

fn read_problem(words: &mut WordsIterator) -> Result<Option<Problem>> {
  if let Some(Ok(problem_size)) = words.next() {
    let problem_size: usize = problem_size.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid problem size"))?;

    if problem_size >= 3 {
      let mut points: Vec<(f64, f64)> = Vec::new();

      for i in 0..problem_size {
        let x: f64 = if let Some(Ok(x)) = words.next() {
          x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid coordinate"))?
        } else {
          return Err(io::Error::new(io::ErrorKind::InvalidData, "expected coordinate"));
        };

        let y: f64 = if let Some(Ok(y)) = words.next() {
          y.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid coordinate"))?
        } else {
          return Err(io::Error::new(io::ErrorKind::InvalidData, "expected coordinate"));
        };

        points.push((x, y));
      }

      return Ok(Some(Problem {
        points,
      }));
    }
  }

  return Ok(None);
}

fn main() -> Result<()> {
  let mut stdin = io::stdin().lock();
  let mut words = WordsIterator::new(&mut stdin);

  while let Some(problem) = read_problem(&mut words)? {
    println!("READ ONE {}", problem.points.len());
  }

  return Ok(());
}
