// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=943
//
// todo: inputs are going to be integers, we might be able to avoid some corner cases where
// floating point introduces them if we wait to convert to float as long as possible.
//
// 

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

      for _ in 0..problem_size {
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
  let mut stdin = io::stdin().lock();
  let mut words = WordsIterator::new(&mut stdin);

  while let Some(problem) = read_problem(&mut words)? {
    print_solution(problem);
  }

  return Ok(());
}
