use std::cmp::Ordering;

use hashbrown::{ HashMap, HashSet };
use itertools::Itertools;

fn is_valid(rules: &HashMap<u8, HashSet<u8>>, update: &Vec<u8>) -> usize {
  let mut agg = HashSet::<u8>::new();
  for x in update.iter() {
    match rules.get(x) {
      Some(must_after) => if agg.intersection(must_after).count() != 0 { return 0; } else { agg.insert(*x); }
      None => { agg.insert(*x); }
    }
  }
  return update[(update.len() - 1) / 2] as usize;
}

fn make_valid(rules: &HashMap<u8, HashSet<u8>>, update: &Vec<u8>) -> usize {
  let result: Vec<u8> = update
    .clone()
    .into_iter()
    .sorted_by(|a: &u8, b: &u8| {
      match rules.get(b) {
        Some(after) => if after.contains(a) { return Ordering::Greater; }
        None => {}
      }
      match rules.get(a) {
        Some(after) => if after.contains(b) { Ordering::Less } else { Ordering::Equal }
        None => { Ordering::Less }
      }
    })
    .collect_vec();
  is_valid(rules, &result)
}

#[aoc23::main(05)]
fn main(input: &str) -> (usize, usize) {
  let (_rules, _reports) = input.split_once("\n\n").unwrap();
  let rules: HashMap<u8, HashSet<u8>> = _rules
    .split('\n')
    .map(|l| l.split_once('|').unwrap())
    .map(|(f, l)| (f.parse::<u8>().unwrap(), l.parse::<u8>().unwrap()))
    .into_group_map()
    .into_iter()
    .map(|(k, v)| (k, v.into_iter().collect()))
    .collect();

  let updates = _reports.split('\n').map(|l|
    l.split(',').map(|x| x.parse::<u8>().unwrap()).collect_vec()
  );
  let (p1, p2) = updates
    .map(|update| {
      let r = is_valid(&rules, &update);
      if r > 0 { (r, 0) } else { (0, make_valid(&rules, &update)) }
    })
    .fold((0, 0), |acc, res| (acc.0 + res.0, acc.1 + res.1));
  (p1, p2)
}
