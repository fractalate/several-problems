// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=943
//
// This problem asks us to find the center of a convex polygon. We do this by doing part
// of a Gram Scan on the points (to order them) and split the polygon into a fan of
// triangles. Then we find a weighted average of the midpoints of the triangles which is
// the center of mass we're looking for.

use std::{collections::VecDeque, io::{self, BufRead, Lines, Result, StdinLock}};

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
  points: Vec<(i32, i32)>,
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

  let mut points: Vec<(i32, i32)> = Vec::new();

  for _ in 0..problem_size {
    let x: i32 = match words.next() {
      Some(Ok(x)) => x.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid coordinate"))?,
      Some(Err(x)) => Err(x)?,
      None => Err(io::Error::new(io::ErrorKind::InvalidData, "expected coordinate"))?,
    };

    let y: i32 = match words.next() {
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
  // Pick the reference point: least y (and least x if there are multiple least y).
  let mut px = problem.points[0].0;
  let mut py = problem.points[0].1;
  for (x, y) in (&problem.points).into_iter().skip(1) {
    if *y < py || (*y == py && *x < px) {
      px = *x;
      py = *y;
    }
  }

  // Order points by the angle/slope they make relative to the reference point.
  problem.points.sort_by(|(ax, ay), (bx, by)| {
    let dxa = ax - px;
    let dya = ay - py;
    let dxb = bx - px;
    let dyb = by - py;

    if dya * dxb < dyb * dxa {
      return std::cmp::Ordering::Less; // Ordered by slope.
    } else if dya * dxb > dyb * dxa {
      return std::cmp::Ordering::Greater; // Ordered by slope.
    } else if dxa + dya < dxb + dyb {
      return std::cmp::Ordering::Less; // Ordered by slope and distance.
    } else if dxa + dya > dxb + dyb {
      return std::cmp::Ordering::Greater; // Ordered by slope and distance.
    }

    return std::cmp::Ordering::Equal;
  });

  // Use one point as the root of our triangle fan.
  let (rx, ry) = problem.points[0];

  let mut total_x = 0.0;
  let mut total_y = 0.0;
  let mut total_area = 0.0;

  // Take two points at a time to complete each triangle fan.
  for i in 1..problem.points.len() - 1 {
    let (ax, ay) = problem.points[i];
    let (bx, by) = problem.points[i + 1];

    let (x, y) = center_of_triangle(ax, ay, bx, by, rx, ry);
    let area = area_of_triangle(ax, ay, bx, by, rx, ry);

    total_x += x * area;
    total_y += y * area;
    total_area += area;
  }

  let (sx, sy) = (total_x / total_area, total_y / total_area);

  println!("{sx:.3} {sy:.3}");
}

fn center_of_triangle(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32) -> (f64, f64) {
  return ((ax + bx + cx) as f64 / 3.0, (ay + by + cy) as f64 / 3.0);
}

fn area_of_triangle(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32) -> f64 {
  return ((ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)) as f64 / 2.0).abs();
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
