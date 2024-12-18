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

fn fill(grid: &mut Vec<Vec<bool>>, bytes: &Vec<(usize,usize)>, start: usize, end: usize) {
  for k in start..end {
    grid[bytes[k].1][bytes[k].0] = true;
  }
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
  fill(&mut grid, &bytes, 0, bytes_init);
  let mut cache = vec![vec![0_usize; size]; size];
  let p1 = step(&grid, &mut cache, size, (0, 0), 1, 1).unwrap() - 1;

  let mut lower = bytes_init;
  let mut upper = bytes.len();

  while upper - lower != 1 {
    let mid = (upper + lower) / 2;
    let mut new_grid = grid.clone();
    fill(&mut new_grid, &bytes, lower, mid);
    cache = vec![vec![0_usize; size]; size];
    match step(&new_grid, &mut cache, size, (0, 0), 1, 1) {
      Some(_) => {
        lower = mid;
        grid = new_grid;
      }
      None => {
        upper = mid;
      }
    }
  }

  let (c, r) = bytes[lower];
  (p1, format!("{},{}", c, r))
}
