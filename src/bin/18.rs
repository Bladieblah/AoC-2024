use hashbrown::HashMap;
use itertools::Itertools;

fn next(pos: (usize, usize), dir: usize) -> (usize, usize) {
  match dir {
    0 => (pos.0 - 1, pos.1), // North
    1 => (pos.0, pos.1 + 1), // East
    2 => (pos.0 + 1, pos.1), // South
    3 => (pos.0, pos.1 - 1), // West
    _ => unreachable!(),
  }
}

fn l(d: usize) -> usize {
  (d + 3) % 4
}
fn r(d: usize) -> usize {
  (d + 1) % 4
}

fn check_cache(cache: &mut Vec<Vec<usize>>, pos: (usize, usize), score: usize) -> bool {
  let val = cache[pos.0][pos.1];
  if val > 0 && val <= score {
    true
  } else {
    cache[pos.0][pos.1] = score;
    false
  }
}

fn step(
  grid: &Vec<Vec<bool>>,
  cache: &mut Vec<Vec<usize>>,
  size: usize,
  pos: (usize, usize),
  dir: usize,
  total: usize
) -> Option<usize> {
  if pos == (size - 1, size - 1) {
    return Some(total);
  }
  let result = vec![dir, l(dir), r(dir)]
    .iter()
    .map(|new_dir| {
      let next_pos = next(pos, *new_dir);
      if
        next_pos.0 >= size ||
        next_pos.1 >= size ||
        grid[next_pos.0][next_pos.1] ||
        check_cache(cache, next_pos, total + 1)
      {
        None
      } else {
        step(grid, cache, size, next_pos, *new_dir, total + 1)
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .min();

  result
}

fn find_paths(
  paths: &mut Vec<Vec<bool>>,
  cache: &Vec<Vec<usize>>,
  pos: (usize, usize),
  dir: usize,
  total: usize,
  size: usize
) {
  vec![dir, l(dir), r(dir)]
    .iter()
    .for_each(|next_dir| {
      let next_pos = next(pos, *next_dir);
      if
        next_pos.0 < size &&
        next_pos.1 < size &&
        !paths[next_pos.0][next_pos.1] &&
        cache[next_pos.0][next_pos.1] == total - 1
      {
        paths[next_pos.0][next_pos.1] = true;
        find_paths(paths, cache, next_pos, *next_dir, total - 1, size);
      }
    });
}

fn get_dist_map(cache: &Vec<Vec<usize>>) -> HashMap<usize,usize> {
  cache
    .iter()
    .flat_map(|x| x)
    .group_by(|x| **x)
    .into_iter()
    .map(|(k, v)| (k, v.count()))
    .collect()
}

#[aoc24::main(18)]
fn main(input: &str) -> (usize, String) {
  let bytes = input
    .split('\n')
    .map(|line| {
      let _bytes = line.split_once(',').unwrap();
      (_bytes.0.parse::<usize>().unwrap(), _bytes.1.parse::<usize>().unwrap())
    })
    .collect_vec();
  let size = 71;
  let bytes_init = 1024;
  let mut grid = vec![vec![false; size]; size];
  let mut cache = vec![vec![0_usize; size]; size];
  cache[0][0] = 1;
  for k in 0..bytes_init {
    grid[bytes[k].1][bytes[k].0] = true;
  }
  // for line in grid.iter() {
  //   for c in line {
  //     print!("{}", if *c {'x'} else {'.'});
  //   }
  //   println!();
  // }
  let mut length = step(&grid, &mut cache, size, (0, 0), 1, 1);
  let p1 = length.unwrap() - 1;
  let mut byte_count = bytes_init;
  let mut paths = vec![vec![false; size]; size];
  find_paths(&mut paths, &cache, (size - 1, size - 1), 3, length.unwrap(), size);
  let mut dists = get_dist_map(&cache);

  while length.is_some() && byte_count < bytes.len() {
    let (c, r) = bytes[byte_count];
    grid[r][c] = true;
    byte_count += 1;
    print!("({:?}, {}) {} -> ", (c,r), cache[r][c], dists[&cache[r][c]]);
    *dists.get_key_value_mut(&cache[r][c]).unwrap().1 -= 1;
    println!("{}", dists[&cache[r][c]]);
    if dists[&cache[r][c]] == 0 {
      println!("Regenerating at byte count {}", byte_count);
      cache = vec![vec![0_usize; size]; size];
      cache[0][0] = 1;
      length = step(&grid, &mut cache, size, (0, 0), 1, 1);
      if length.is_some() {
        paths = vec![vec![false; size]; size];
        find_paths(&mut paths, &cache, (size - 1, size - 1), 3, length.unwrap(), size);
        dists = get_dist_map(&cache);
      }
    }
  }

  let (c, r) = bytes[byte_count];
  (p1, format!("{},{}", c, r))
}
