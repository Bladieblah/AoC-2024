use itertools::Itertools;

fn check_side(grid: &Vec<&[u8]>, size: usize, pos: (usize,usize), next_pos: (usize,usize), cur: u8, dir: usize) -> usize {
  let prev1 = match dir {
    0 => { if pos.0 == 0 { return 1 }; (pos.0 - 1, pos.1)},
    1 => { if pos.1 == 0 { return 1 }; (pos.0, pos.1 - 1)},
    _ => unreachable!()
  };

  let prev = grid[prev1.0][prev1.1];
  if prev != cur {
    return 1;
  }

  let prev2 = match dir {
    0 => {
      if next_pos.1 >= size { return 0 }
      (next_pos.0 - 1, next_pos.1)
    },
    1 => {
      if next_pos.0 >= size { return 0 }
      (next_pos.0, next_pos.1 - 1)
    },
    _ => unreachable!()
  };

  (prev == grid[prev2.0][prev2.1]) as usize
}

fn search_island(grid: &Vec<&[u8]>, seen: &mut Vec<Vec<bool>>, size: usize, pos: (usize,usize), cur: u8) -> (usize, usize, usize) {
  let mut area = 1;
  let mut perim = 0;
  let mut sides = 0;

  seen[pos.0][pos.1] = true;

  for (next_pos, dir) in vec![
    ((pos.0 + 1, pos.1), 1),
    ((pos.0 - 1, pos.1), 1),
    ((pos.0, pos.1 + 1), 0),
    ((pos.0, pos.1 - 1), 0),
  ] {
    if next_pos.0 >= size || next_pos.1 >= size {
      perim += 1;
      sides += check_side(grid, size, pos, next_pos, cur, dir);
      continue;
    }
    let next = grid[next_pos.0][next_pos.1];
    if next == cur {
      if !seen[next_pos.0][next_pos.1] {
        let (a, p, s) = search_island(grid, seen, size, next_pos, next);
        area += a;
        perim += p;
        sides += s;
      }
    } else {
      perim += 1;
      sides += check_side(grid, size, pos, next_pos, cur, dir);
    }
  }

  (area, perim, sides)
}

#[aoc24::main(12)]
fn main(input: &str) -> (usize, usize) {
  let grid = input
    .split('\n')
    .map(|line| line.as_bytes())
    .collect_vec();
  let size = grid.len();
  let mut seen = vec![vec![false; size]; size];

  (0..size).cartesian_product(0..size)
    .map(|pos| {
      if !seen[pos.0][pos.1] {
        let res = search_island(&grid, &mut seen, size, pos, grid[pos.0][pos.1]);
        (res.0 * res.1, res.0 * res.2)
      } else {
        (0, 0)
      }
    })
    .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}
