// UVA-10685 - Nature
//
// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=24&page=show_problem&problem=1626
//
// Essentially we are given a directed graph (which may have cycles) and are asked to find
// the length of the longest path of connected nodes representing the length of the longest
// predator/prey chain.

use std::collections::{HashMap, HashSet};
use std::io::{self, Lines, StdinLock};
use std::io::BufRead;

fn read_problem_statement(lines: &mut Lines<StdinLock<'static>>) -> Option<(usize, usize)> {
  while let Some(Ok(line)) = lines.next() {
    if line.len() == 0 {
      continue;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() == 2, "invalid problem statement");

    let count_nodes: usize = parts[0].parse().expect("invalid node count");
    let count_edges: usize = parts[1].parse().expect("invalid edge count");

    return Some((count_nodes, count_edges));
  }

  return None
}

fn read_nodes(lines: &mut Lines<StdinLock<'static>>, count_nodes: usize) -> Vec<String> {
  let mut result: Vec<String> = Vec::new();

  for _ in 0..count_nodes {
    if let Some(Ok(line)) = lines.next() {
      result.push(line.to_string());
    }
  }

  return result;
}

fn read_edges(lines: &mut Lines<StdinLock<'static>>, nodes: &Vec<String>, count_edges: usize) -> HashMap<usize, Vec<usize>> {
  let mut edges: HashMap<usize, Vec<usize>> = HashMap::new();

  for _ in 0..count_edges {
    if let Some(Ok(line)) = lines.next() {
      let parts: Vec<&str> = line.split_whitespace().collect();
      assert!(parts.len() == 2, "invalid edge");

      let node = nodes.iter().position(|x| x == parts[0]).expect("invalid node name");
      let parent = nodes.iter().position(|x| x == parts[1]).expect("invalid parent node name");

      if let Some(list) = edges.get_mut(&parent) {
        list.push(node);
      } else {
        edges.insert(parent, Vec::from([node]));
      }
    }
  }

  return edges;
}

fn calculate_depth(edges: &HashMap<usize, Vec<usize>>, node: usize, seen: &mut HashSet<usize>) -> usize {
  let mut max_depth = 0;

  if seen.insert(node) {
    if let Some(children) = edges.get(&node) {
      for child in children {
        max_depth = max_depth.max(calculate_depth(edges, *child, seen));
      }
    }
    max_depth += 1;
    seen.remove(&node);
  }

  return max_depth;
}

fn calculate_max_depth(edges: &HashMap<usize, Vec<usize>>, count_nodes: usize) -> usize {
  let mut seen: HashSet<usize> = HashSet::new();
  let mut max_depth = 0;

  for node in 0..count_nodes {
    max_depth = max_depth.max(calculate_depth(edges, node, &mut seen));
  }

  return max_depth;
}

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some((count_nodes, count_edges)) = read_problem_statement(&mut lines) {
    if count_nodes == 0 {
      break;
    }

    let nodes = read_nodes(&mut lines, count_nodes);
    let edges = read_edges(&mut lines, &nodes, count_edges);
    let max_depth = calculate_max_depth(&edges, count_nodes);

    println!("{}", max_depth);
  }
}
