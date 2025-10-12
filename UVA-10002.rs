// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=943
//
// todo: inputs are going to be integers, we might be able to avoid some corner cases where
// floating point introduces them if we wait to convert to float as long as possible.
//
// 

use std::{collections::VecDeque, io::{self, BufRead, Lines, Result, StdinLock}};

// Maybe some day we don't just read the whole input, but instead read portions and produce the words.
struct WordsIterator<'a> {
  lines: &'a mut Lines<StdinLock<'static>>,
  words: VecDeque<String>,
}

impl<'a> WordsIterator<'a> {
  fn new(lines: &'a mut Lines<StdinLock<'static>>) -> WordsIterator<'a> {
    WordsIterator {
      lines,
      words: VecDeque::new(),
    }
  }

  fn read_words(&mut self) -> Result<()> {
    let result = self.lines.next();

    if let Some(Err(x)) = result {
      return Err(x);
    }

    if let Some(Ok(line)) = result {
      let parts = line.split_whitespace().filter(|x| x.len() > 0).map(|x| x.into());
      self.words.extend(parts);
    }

    return Ok(());
  }
}

impl<'a> Iterator for WordsIterator<'a> {
  type Item = Result<String>;

  fn next(&mut self) -> Option<Result<String>> {
    if let Some(word) = self.words.pop_front() {
      return Some(Ok(word));
    }

    if let Err(x) = self.read_words() {
      return Some(Err(x));
    }

    if let Some(word) = self.words.pop_front() {
      return Some(Ok(word));
    }

    return None;
  }
}

struct Problem {
  points: Vec<(f64, f64)>,
}

fn read_problem(words: &mut WordsIterator) -> Result<Option<Problem>> {
  let problem_size: usize = match words.next() {
    Some(Ok(x)) => x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid problem size"))?,
    Some(Err(x)) => Err(x)?,
    None => return Ok(None),
  };

  if problem_size < 3 {
    return Ok(None);
  }

  let mut points: Vec<(f64, f64)> = Vec::new();

  for _ in 0..problem_size {
    let x: f64 = match words.next() {
      Some(Ok(x)) => x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid coordinate"))?,
      Some(Err(x)) => Err(x)?,
      None => Err(io::Error::new(io::ErrorKind::InvalidData, "expected coordinate"))?,
    };

    let y: f64 = match words.next() {
      Some(Ok(x)) => x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid coordinate"))?,
      Some(Err(x)) => Err(x)?,
      None => Err(io::Error::new(io::ErrorKind::InvalidData, "expected coordinate"))?,
    };

    points.push((x, y));
  }

  return Ok(Some(Problem {
    points,
  }));
}

fn print_solution(mut problem: Problem) {
  let mut px = problem.points[0].0;
  let mut py = problem.points[0].1;
  for (x, y) in (&problem.points).into_iter().skip(1) {
    if *y < py || (*y == py && *x < px) {
      px = *x;
      py = *y;
    }
  }

  problem.points.sort_by(|(ax, ay), (bx, by)| {
    let dxa = ax - px;
    let dya = ay - py;
    let dxb = bx - px;
    let dyb = by - py;

    let ma = if dxa == 0.0 && dya == 0.0 { 0.0 } else { dya / dxa };
    let mb = if dxb == 0.0 && dyb == 0.0 { 0.0 } else { dyb / dxb };

    if ma < mb {
      return std::cmp::Ordering::Less;
    } else if ma > mb {
      return std::cmp::Ordering::Greater;
    } else if dxa + dya < dxb + dyb {
      return std::cmp::Ordering::Less;
    } else if dxa + dya > dxb + dyb {
      return std::cmp::Ordering::Greater;
    }

    return std::cmp::Ordering::Equal;
  });

  let (rx, ry) = problem.points[0];

  let mut tx = 0.0;
  let mut ty = 0.0;
  let mut ta = 0.0;

  for i in 1..problem.points.len() - 1 {
    let (ax, ay) = problem.points[i];
    let (bx, by) = problem.points[i + 1];

    let (x, y) = center_of_triangle(ax, ay, bx, by, rx, ry);
    let area = area_of_triangle(ax, ay, bx, by, rx, ry);

    tx += x * area;
    ty += y * area;
    ta += area;
  }

  let (sx, sy) = (tx / ta, ty / ta);

  println!("{sx:.3} {sy:.3}");
}

fn center_of_triangle(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> (f64, f64) {
  return ((ax + bx + cx) / 3.0, (ay + by + cy) / 3.0);
}

fn area_of_triangle(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
  return ((ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)) / 2.0).abs();
}

fn main() -> Result<()> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut words = WordsIterator::new(&mut lines);

  while let Some(problem) = read_problem(&mut words)? {
    print_solution(problem);
  }

  return Ok(());
}
