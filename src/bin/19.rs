use hashbrown::HashMap;
use itertools::Itertools;

fn check_towel(pattern: &Vec<u8>, plen: usize, towel: &Vec<u8>, index: usize) -> bool {
  for (i, color) in towel.iter().enumerate() {
    if index + i >= plen || pattern[index + i] != *color {
      return false;
    }
  }
  true
}

fn make_pattern(
  pattern: &Vec<u8>,
  plen: usize,
  towels: &HashMap<u8, Vec<Vec<u8>>>,
  index: usize,
  cache: &mut Vec<Option<usize>>
) -> usize {
  if index == plen {
    return 1;
  }
  match cache[index] {
    Some(value) => {
      return value;
    }
    None => (),
  }
  let mut count = 0;

  match towels.get(&pattern[index]) {
    None => (),
    Some(value) => {
      for towel in value {
        if check_towel(pattern, plen, towel, index) {
          count += make_pattern(pattern, plen, towels, index + towel.len(), cache);
        }
      }
    }
  }
  cache[index] = Some(count);
  count
}

#[aoc24::main(19)]
fn main(input: &str) -> (usize, usize) {
  let (_towels, _patterns) = input.split_once("\n\n").unwrap();
  let towels: HashMap<u8, Vec<Vec<u8>>> = _towels
    .split(", ")
    .map(|s| s.as_bytes().to_vec())
    .sorted_by(|a, b| a[0].cmp(&b[0]))
    .group_by(|towel| towel[0])
    .into_iter()
    .map(|(k, g)| (k, g.sorted_by(|a, b| b.len().cmp(&a.len())).collect_vec()))
    .collect();
  let patterns = _patterns
    .split('\n')
    .map(|p| p.as_bytes().to_vec())
    .collect_vec();
  patterns
    .iter()
    .map(|pattern| {
      let count = make_pattern(pattern, pattern.len(), &towels, 0, &mut vec![None; pattern.len()]);
      ((count > 0) as usize, count)
    })
    .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}
