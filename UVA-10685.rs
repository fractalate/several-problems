// UVA-10685 - Nature
//
// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=24&page=show_problem&problem=1626

use std::collections::{HashMap, HashSet};
use std::io::{self, Lines, StdinLock};
use std::io::BufRead;

fn read_vegetals(lines: &mut Lines<StdinLock<'static>>, count_vegetals: usize) -> Vec<String> {
  let mut result: Vec<String> = Vec::new();

  for _ in 0..count_vegetals {
    if let Some(Ok(line)) = lines.next() {
      result.push(line.to_string());
    }
  }

  return result;
}

fn read_links(lines: &mut Lines<StdinLock<'static>>, vegetals: &Vec<String>, count_links: usize) -> HashMap<usize, Vec<usize>> {
  let mut links: HashMap<usize, Vec<usize>> = HashMap::new();

  for _ in 0..count_links {
    if let Some(Ok(line)) = lines.next() {
      let parts: Vec<&str> = line.split_whitespace().collect();

      assert!(parts.len() == 2);
      let prey = parts[0];
      let predator = parts[1];

      let prey_id = vegetals.iter().position(|x| x == prey).unwrap();
      let predator_id = vegetals.iter().position(|x| x == predator).unwrap();

      if let Some(list) = links.get_mut(&predator_id) {
        list.push(prey_id);
      } else {
        links.insert(predator_id, Vec::from([prey_id]));
      }
    }
  }

  return links;
}

fn calculate_food_chain_depth(links: &HashMap<usize, Vec<usize>>, predator_id: usize) -> usize {
  let mut seen: HashSet<usize> = HashSet::new();
  return _calculate_food_chain_depth(links, predator_id, &mut seen);
}

fn _calculate_food_chain_depth(links: &HashMap<usize, Vec<usize>>, predator_id: usize, seen: &mut HashSet<usize>) -> usize {
  if !seen.insert(predator_id) {
    return 0;
  }
  let mut max_depth = 0;
  if let Some(prey_ids) = links.get(&predator_id) {
    for prey_id in prey_ids {
      max_depth = max_depth.max(_calculate_food_chain_depth(links, *prey_id, seen));
    }
  }
  seen.remove(&predator_id);
  return max_depth + 1;
}

fn calculate_longest_food_chain_depth(count_vegetals: usize, links: &HashMap<usize, Vec<usize>>) -> usize {
  let mut maximum_depth = 0;
  for predator_id in 0..count_vegetals {
    maximum_depth = usize::max(maximum_depth, calculate_food_chain_depth(links, predator_id))
  }
  return maximum_depth;
}

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  while let Some(Ok(line)) = lines.next() {
    if line.len() == 0 {
      continue;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();

    assert!(parts.len() == 2);
    let count_vegetals: usize = parts[0].parse().unwrap();
    let count_links: usize = parts[1].parse().unwrap();

    if count_vegetals == 0 && count_links == 0 {
      break;
    }

    let vegetals = read_vegetals(&mut lines, count_vegetals);
    let links = read_links(&mut lines, &vegetals, count_links);
  
    let radius = calculate_longest_food_chain_depth(count_vegetals, &links);
    println!("{}", radius);
  }
}
