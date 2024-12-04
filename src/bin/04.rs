use std::sync::OnceLock;
use itertools::Itertools;

fn dirs() -> &'static Vec<(usize, usize)> {
  static DIRS: OnceLock<Vec<(usize, usize)>> = OnceLock::new();
  DIRS.get_or_init(||
    vec![
      (0, 1),
      (0, usize::MAX),
      (1, 0),
      (usize::MAX, 0),
      (1, 1),
      (usize::MAX, usize::MAX),
      (1, usize::MAX),
      (usize::MAX, 1)
    ]
  )
}

fn step(chars: &Vec<&[u8]>, size: usize, pos: (usize, usize), dir: (usize, usize), cur: u8) -> usize {
  let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
  if next_pos.0 >= size || next_pos.1 >= size {
    return 0;
  }
  let next = chars[next_pos.0][next_pos.1];

  match (cur, next) {
    (b'X', b'M') => step(chars, size, next_pos, dir, next),
    (b'M', b'A') => step(chars, size, next_pos, dir, next),
    (b'A', b'S') => 1,
    _ => 0,
  }
}

fn find_xmas(chars: &Vec<&[u8]>, size: usize, start: (usize, usize)) -> usize {
  dirs()
    .into_iter()
    .map(|dir| step(chars, size, start, *dir, b'X'))
    .sum()
}

fn find_masmas(chars: &Vec<&[u8]>, i: usize, j: usize) -> usize {
  let corners = ((chars[i + 1][j + 1], chars[i - 1][j - 1]), (chars[i - 1][j + 1], chars[i + 1][j - 1]));
  match corners.0 {
    (b'M', b'S') | (b'S', b'M') =>
      match corners.1 {
        (b'M', b'S') | (b'S', b'M') => 1,
        _ => 0,
      }
    _ => 0,
  }
}

#[aoc23::main(04)]
fn main(input: &str) -> (usize, usize) {
  let chars = input
    .split('\n')
    .map(|line| line.as_bytes())
    .collect_vec();
  let size = chars.len();
  let p1 = (0..size)
    .cartesian_product(0..size)
    .into_iter()
    .filter(|(i, j)| chars[*i][*j] == b'X')
    .map(|pos| find_xmas(&chars, size, pos))
    .sum();

  let p2 = (1..size - 1)
    .cartesian_product(1..size - 1)
    .into_iter()
    .filter(|(i, j)| chars[*i][*j] == b'A')
    .map(|(i, j)| find_masmas(&chars, i, j))
    .sum();

  (p1, p2)
}
