// UVA-10000 - Longest Path
//
// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=941
//
// We're given a connected, directed graph with no cycles and we want to find the longest
// path starting at a given node.

use std::collections::{
  HashSet,
};
use std::io::{
  self,
  BufRead,
  Lines,
  StdinLock,
};

#[derive(Debug)]
enum ReadProblemError {
  NoMoreLines,
  InvalidFormat,
  ParseError,
}

fn read_usize(lines: &mut Lines<StdinLock<'static>>) -> Result<usize, ReadProblemError> {
  if let Some(Ok(line)) = lines.next() {
    return line.parse::<usize>().map_err(|_| ReadProblemError::ParseError);
  }
  return Err(ReadProblemError::NoMoreLines);
}

fn read_usize_pair(lines: &mut Lines<StdinLock<'static>>) -> Result<(usize, usize), ReadProblemError> {
  if let Some(Ok(line)) = lines.next() {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
      return Err(ReadProblemError::InvalidFormat);
    }
    let left_value = parts[0].parse::<usize>().map_err(|_| ReadProblemError::ParseError)?;
    let right_value = parts[1].parse::<usize>().map_err(|_| ReadProblemError::ParseError)?;
    return Ok((left_value, right_value));
  }
  return Err(ReadProblemError::NoMoreLines);
}

struct Problem {
  count_nodes: usize,
  start_node: usize,
  edges_out: Vec<HashSet<usize>>,
  edges_in: Vec<HashSet<usize>>,
}

fn read_problem(lines: &mut Lines<StdinLock<'static>>) -> Result<Option<Problem>, ReadProblemError> {
  let count_nodes = read_usize(lines)?;

  if count_nodes == 0 {
    return Ok(None);
  }

  let start_node = read_usize(lines)?;
  let mut edges_out: Vec<HashSet<usize>> = vec![HashSet::new(); count_nodes];
  let mut edges_in: Vec<HashSet<usize>> = vec![HashSet::new(); count_nodes];

  loop {
    let (parent_node, child_node) = read_usize_pair(lines)?;
    if parent_node == 0 || child_node == 0 {
      break;
    }
    edges_out[parent_node - 1].insert(child_node - 1);
    edges_in[child_node - 1].insert(parent_node - 1);
  }

  return Ok(Some(Problem {
    count_nodes,
    start_node: start_node - 1,
    edges_out,
    edges_in,
  }));
}

#[derive(Clone)]
struct GraphNode {
  degree_in: usize,
  depth: Option<usize>,
}

fn print_solution(case_number: usize, problem: &Problem) {
  let mut stack: Vec<usize> = Vec::new();
  let mut nodes: Vec<GraphNode> = Vec::new();
  nodes.reserve(problem.count_nodes);

  for node in 0..problem.count_nodes {
    let degree_in = problem.edges_in[node].len();
    let depth = if node == problem.start_node {
      Some(0)
    } else {
      None
    };

    nodes.push(GraphNode {
      degree_in,
      depth,
    });

    if degree_in == 0 {
      stack.push(node);
    }
  }

  let mut maximum_depth = 0;
  let mut maximum_depth_node = problem.start_node;

  while let Some(parent_node) = stack.pop() {
    for child_node in &problem.edges_out[parent_node] {
      if let Some(depth) = nodes[parent_node].depth {
        let next_depth = nodes[*child_node].depth.unwrap_or(0).max(depth + 1);
        if next_depth > maximum_depth || (next_depth == maximum_depth && *child_node < maximum_depth_node) {
          maximum_depth = next_depth;
          maximum_depth_node = *child_node;
        }
        nodes[*child_node].depth = Some(next_depth);
      }

      nodes[*child_node].degree_in -= 1;
      if nodes[*child_node].degree_in == 0 {
        stack.push(*child_node);
      }
    }
  }

  println!(
    "Case {}: The longest path from {} has length {}, finishing at {}.",
    case_number,
    problem.start_node + 1,
    maximum_depth,
    maximum_depth_node + 1,
  );
}

fn main() -> Result<(), ReadProblemError> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut case_number: usize = 1;

  while let Some(problem) = read_problem(&mut lines)? {
    print_solution(case_number, &problem);
    case_number += 1;
  }

  return Ok(());
}
