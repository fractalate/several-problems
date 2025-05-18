use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;
use std::ops::Div;

struct RuleSet {
  ordering_rules: HashMap<usize, HashSet<usize>>,
}

impl RuleSet {
  fn new() -> RuleSet {
    return RuleSet {
      ordering_rules: HashMap::new(),
    };
  }

  fn add_rule(&mut self, before: usize, after: usize) {
    if let Some(others) = self.ordering_rules.get_mut(&before) {
      others.insert(after);
    } else {
      let mut others = HashSet::new();
      others.insert(after);
      self.ordering_rules.insert(before, others);
    }
    if !self.ordering_rules.contains_key(&after) {
      self.ordering_rules.insert(after, HashSet::new());
    }
  }

  fn contains_rule_containing_page(&self, page: usize) -> bool {
    return self.ordering_rules.contains_key(&page);
  }

  fn contains_rule(&self, before: usize, after: usize) -> bool {
    if let Some(others) = self.ordering_rules.get(&before) {
      return others.contains(&after);
    }
    return false;
  }
}

fn true_if_incorrect_but_order_update(rules: &RuleSet, update: &mut Vec<usize>) -> bool {
  let orig = update.clone();
  update.sort_by(|before, after| if rules.contains_rule_containing_page(*before) && rules.contains_rule_containing_page(*after) {
    if before == after {
      Ordering::Equal
    } else if rules.contains_rule(*before, *after) {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  } else {
    Ordering::Equal
  });
  for i in 0..orig.len() {
    if orig[i] != update[i] {
      return true;
    }
  }
  return false;
}

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  // Read the rules.
  let mut rules = RuleSet::new();
  while let Some(Ok(line)) = lines.next() {
    if line.is_empty() {
      break; // Done with the rule section.
    }
    let parts: Vec<usize> = line.split("|").map(|x| x.parse::<usize>().unwrap()).collect();
    assert!(parts.len() > 1);
    rules.add_rule(parts[0], parts[1]);
  }
  let rules = rules;

  let mut ttl = 0;

  // Read the updates.
  while let Some(Ok(line)) = lines.next() {
    let mut parts: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    assert!(parts.len() > 0);
    assert!(parts.len() % 2 != 0);

    
    if true_if_incorrect_but_order_update(&rules, &mut parts) {
      let median = parts[parts.len().div(2)];
      ttl += median;
    }
  }

  println!("{}", ttl);

}
