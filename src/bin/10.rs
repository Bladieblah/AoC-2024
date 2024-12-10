use hashbrown::HashSet;
use itertools::Itertools;

fn step(map: &Vec<Vec<u8>>, size: usize, pos: (usize, usize), val: u8, ends: &mut HashSet<(usize,usize)>) -> usize {
  if val == 9 {
    ends.insert(pos);
    return 1;
  }
  vec![(pos.0,pos.1+1), (pos.0,pos.1-1), (pos.0+1,pos.1), (pos.0-1,pos.1)].into_iter().map(|next| {
    if next.0 < size && next.1 < size {
      if map[next.0][next.1] == val + 1 {
        return step(map, size, next, val + 1, ends)
      }
    }
    0
  }).sum()
}

#[aoc24::main(10)]
fn main(input: &str) -> (usize, usize) {
  let map = input.split('\n').map(|line| line.as_bytes().into_iter().map(|x| x - b'0').collect_vec()).collect_vec();
  let size = map.len();
  (0..size).cartesian_product(0..size).filter(|(i,j)| map[*i][*j] == 0).map(|start| {
    let mut ends = HashSet::<(usize,usize)>::new();
    let _p2 = step(&map, size, start, 0, &mut ends);
    (ends.len(), _p2)
  })
  .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}
