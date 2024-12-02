use std::iter::zip;

use hashbrown::HashMap;
use itertools::Itertools;

#[aoc23::main(01)]
fn main(input: &str) -> (usize, usize) {
  let (mut ids1, mut ids2): (Vec<usize>, Vec<usize>) = input
    .split('\n')
    .map(|s| s.split_once("   ").unwrap())
    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
    .unzip();

  ids1.sort();
  ids2.sort();

  let p1: usize = zip(&ids1, &ids2)
    .into_iter()
    .map(|(l, r)| (*l).abs_diff(*r))
    .sum();

  let g2: HashMap<usize, usize> = ids2
    .into_iter()
    .group_by(|x| *x)
    .into_iter()
    .map(|(x, g)| (x, g.count()))
    .collect();

  let check_count = |x: usize| {
    match g2.get(&x) {
      Some(y) => x * y,
      None => 0,
    }
  };

  let p2 = ids1
    .into_iter()
    .map(|x| check_count(x))
    .sum();

  (p1, p2)
}
