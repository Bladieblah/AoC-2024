use std::sync::OnceLock;

use itertools::Itertools;

fn dirs() -> &'static Vec<(usize, usize)> {
  static DIRS: OnceLock<Vec<(usize, usize)>> = OnceLock::new();
  DIRS.get_or_init(||
    vec![
      (usize::MAX, 0),
      (0, 1),
      (1, 0),
      (0, usize::MAX),
    ]
  )
}

fn rot(dir: usize) -> usize {
  (dir + 1) % 4
}

fn check_loop(grid: &Vec<Vec<u8>>, start: (usize, usize, usize), size: usize, visited: &mut Vec<Vec<Vec<bool>>>) -> bool {
  let mut pos = start.clone();

  while pos.0 < size && pos.1 < size {
    if visited[pos.0][pos.1][pos.2] {
      return true;
    }
    visited[pos.0][pos.1][pos.2] = true;

    let dir = dirs()[pos.2];
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1, pos.2);

    if new_pos.0 >= size || new_pos.1 >= size {
      return false;
    } else if grid[new_pos.0][new_pos.1] == b'#' {
      pos.2 = rot(pos.2);
    } else {
      pos = new_pos;
    }
  }

  false
}


fn walk(grid: &mut Vec<Vec<u8>>, start: (usize, usize, usize), size: usize) -> (Vec<Vec<Vec<bool>>>, Vec<Vec<bool>>) {
  let mut visited = vec![vec![vec![false; 4]; size]; size];
  let mut new_stones = vec![vec![false; size]; size];

  let mut pos = start.clone();

  while !visited[pos.0][pos.1][pos.2] {
    visited[pos.0][pos.1][pos.2] = true;

    let dir = dirs()[pos.2];
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1, pos.2);

    if new_pos.0 >= size || new_pos.1 >= size {
      return (visited, new_stones);
    } else if grid[new_pos.0][new_pos.1] == b'#' {
      pos.2 = rot(pos.2);
    } else {
      grid[new_pos.0][new_pos.1] = b'#';
      if !visited[new_pos.0][new_pos.1].iter().any(|d| *d) && check_loop(grid, (pos.0, pos.1, rot(pos.2)), size, &mut visited.clone()) {
        new_stones[new_pos.0][new_pos.1] = true;
      }
      grid[new_pos.0][new_pos.1] = b'.';
      pos = new_pos;
    }
  }
  (visited, new_stones)
}

#[aoc23::main(06)]
fn main(input: &str) -> (usize, usize) {
  let mut grid = input.split('\n').map(|line| line.as_bytes().to_vec()).collect_vec();
  let size = grid.len();
  let start: (usize, usize) = (0..size).cartesian_product(0..size).filter(|(i,j)| grid[*i][*j] == b'^').next().unwrap();
  let (visited, new_stones) = walk(&mut grid, (start.0, start.1, 0), size);
  let p1 = visited.iter().flat_map(|row| row).fold(0, |acc, cell| acc + cell.iter().any(|d| *d) as usize);
  let p2 = new_stones.iter().flat_map(|row| row).fold(0, |acc, cell| acc + *cell as usize);

  (p1,p2)
}
