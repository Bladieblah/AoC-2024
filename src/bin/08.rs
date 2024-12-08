use hashbrown::HashSet;
use itertools::Itertools;

fn check_node(
  size: usize,
  antennas: &Vec<(usize, usize)>,
  nodes1: &mut HashSet<(usize, usize)>,
  nodes2: &mut HashSet<(usize, usize)>
) {
  let a = antennas[0];
  let b = antennas[1];

  let delta = (b.0 - a.0, b.1 - a.1);
  let mut x = (a.0 - delta.0, a.1 - delta.1);
  let mut y = (b.0 + delta.0, b.1 + delta.1);

  if x.0 < size && x.1 < size {
    nodes1.insert(x);
    while x.0 < size && x.1 < size {
      nodes2.insert(x);
      x = (x.0 - delta.0, x.1 - delta.1);
    }
  }

  if y.0 < size && y.1 < size {
    nodes1.insert(y);
    while y.0 < size && y.1 < size {
      nodes2.insert(y);
      y = (y.0 + delta.0, y.1 + delta.1);
    }
  }
}

#[aoc24::main(08)]
fn main(input: &str) -> (usize, usize) {
  let mut nodes1 = HashSet::<(usize, usize)>::new();
  let mut nodes2 = HashSet::<(usize, usize)>::new();
  let grid = input
    .split('\n')
    .map(|line| line.as_bytes())
    .collect_vec();
  let size = grid.len();
  (0..size)
    .cartesian_product(0..size)
    .filter(|(i, j)| grid[*i][*j] != b'.')
    .map(|(i, j)| {
      nodes2.insert((i, j));
      (grid[i][j], (i, j))
    })
    .into_group_map()
    .into_iter()
    .for_each(|(_, v)|
      v
        .into_iter()
        .combinations(2)
        .for_each(|antennas| check_node(size, &antennas, &mut nodes1, &mut nodes2))
    );
  (nodes1.len(), nodes2.len())
}
