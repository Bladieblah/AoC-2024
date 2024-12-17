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

fn check_cache(cache: &mut Vec<Vec<Vec<usize>>>, pos: (usize, usize), dir: usize, score: usize) -> bool {
  let val = cache[pos.0][pos.1][dir];
  let val_op = cache[pos.0][pos.1][(dir + 2) % 4];
  if (val > 0 && val <= score) || (val_op > 0 && val_op < score) {
    true
  } else {
    cache[pos.0][pos.1][dir] = score;
    false
  }
}

fn step(
  maze: &Vec<&[u8]>,
  cache: &mut Vec<Vec<Vec<usize>>>,
  size: usize,
  pos: (usize, usize),
  dir: usize,
  total: usize
) -> Option<usize> {
  if pos == (size - 2, 1) {
    return Some(total + 1000 * ((dir != 3) as usize));
  }

  let result = vec![(dir, 1), (l(dir), 1001), (r(dir), 1001)]
    .iter()
    .map(|(d, score)| {
      let next_pos = next(pos, *d);
      if
        (*score != 1 && check_cache(cache, pos, *d, score + total - 1)) ||
        maze[next_pos.0][next_pos.1] == b'#' ||
        check_cache(cache, next_pos, *d, score + total)
      {
        None
      } else {
        step(maze, cache, size, next_pos, *d, total + score)
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .min();

  result
}

fn find_paths(paths: &mut Vec<Vec<bool>>, cache: &Vec<Vec<Vec<usize>>>, pos: (usize, usize), dir: usize, total: usize) {
  vec![(dir, 1), (l(dir), 1001), (r(dir), 1001)]
    .iter()
    .for_each(|(d, score)| {
      let next_pos = next(pos, *d);
      if !paths[next_pos.0][next_pos.1] && cache[next_pos.0][next_pos.1][(*d + 2) % 4] == total - *score {
        paths[next_pos.0][next_pos.1] = true;
        find_paths(paths, cache, next_pos, *d, total - score);
      }
    });
}

#[aoc24::main(16)]
fn main(input: &str) -> (usize, usize) {
  let maze = input
    .split('\n')
    .map(|line| line.as_bytes())
    .collect_vec();
  let size = maze.len();
  let mut cache = vec![vec![vec![0_usize; 4]; size]; size];
  let p1 = vec![(1, size - 3), (2, size - 2)]
    .iter()
    .map(|x| step(&maze, &mut cache, size, *x, 2, 1).unwrap())
    .min()
    .unwrap();
  let mut paths = vec![vec![false; size]; size];
  cache[1][size-3][3] = 1;
  cache[2][size-2][2] = 1;
  find_paths(&mut paths, &cache, (size - 2, 1), 1, p1);
  let p2 = paths.iter().flat_map(|x| x.iter().map(|y| *y as usize)).sum::<usize>() + 2;
  (p1, p2)
}
