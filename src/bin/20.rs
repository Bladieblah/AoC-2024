use itertools::Itertools;

fn get_distmap(maze: &Vec<Vec<u8>>, size: usize, cache: &mut Vec<Vec<usize>>, pos: (usize, usize), dist: usize) {
  if maze[pos.0][pos.1] == b'#' {
    return;
  }
  if cache[pos.0][pos.1] > 0 && cache[pos.0][pos.1] < dist {
    return;
  }
  cache[pos.0][pos.1] = dist;
  for next_pos in vec![(pos.0 + 1, pos.1), (pos.0 - 1, pos.1), (pos.0, pos.1 + 1), (pos.0, pos.1 - 1)] {
    get_distmap(maze, size, cache, next_pos, dist + 1);
  }
}

#[aoc24::main(20)]
fn main(input: &str) -> (usize, usize) {
  let maze = input
    .split('\n')
    .map(|line| line.as_bytes().to_vec())
    .collect_vec();
  let size = maze.len();
  let start = (0..size)
    .cartesian_product(0..size)
    .filter(|(i, j)| maze[*i][*j] == b'S')
    .next()
    .unwrap();

  let mut dists = vec![vec![0_usize; size]; size];
  get_distmap(&maze, size, &mut dists, start, 1);
  let cutoff = size - 2;

  let mut p1 = 0;
  let mut p2 = 0;
  let min_saved = 100;

  for i in 1..size - 1 {
    for j in 1..size - 1 {
      let d1 = dists[i][j];
      if d1 == 0 { continue; }
      for di in 2..21_usize {
        if i + di > cutoff { break; }
        if dists[i + di][j] > 0 && d1.abs_diff(dists[i + di][j]) >= min_saved + di {
          if di == 2 { p1 += 1; }
          p2 += 1;
        }
      }
      for dj in 1..21_usize {
        if j + dj > cutoff { break; }
        if dists[i][j + dj] > 0 && d1.abs_diff(dists[i][j + dj]) >= min_saved + dj {
          if dj == 2 { p1 += 1; }
          p2 += 1;
        }
        for di in 1..21 - dj {
          if i - di <= cutoff && dists[i - di][j + dj] > 0 && d1.abs_diff(dists[i - di][j + dj]) >= min_saved + di + dj {
            p2 += 1;
          }
          if i + di <= cutoff && dists[i + di][j + dj] > 0 && d1.abs_diff(dists[i + di][j + dj]) >= min_saved + di + dj {
            p2 += 1;
          }
        }
      }
    }
  }

  (p1, p2)
}
