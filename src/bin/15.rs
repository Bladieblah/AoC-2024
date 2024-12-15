use itertools::Itertools;

fn step(grid: &mut Vec<Vec<u8>>, pos: (usize, usize), dir: (usize, usize)) -> Option<(usize, usize)> {
  let next = (pos.0 + dir.0, pos.1 + dir.1);
  match grid[next.0][next.1] {
    b'.' => {
      grid[next.0][next.1] = grid[pos.0][pos.1];
      grid[pos.0][pos.1] = b'.';
      Some(next)
    }
    b'#' => None,
    b'O' | b'[' | b']' => {
      if step(grid, next, dir).is_some() {
        grid[next.0][next.1] = grid[pos.0][pos.1];
        grid[pos.0][pos.1] = b'.';
        Some(next)
      } else {
        None
      }
    }
    _ => unreachable!(),
  }
}

fn push(wgrid: &mut Vec<Vec<u8>>, pos: (usize, usize), dir: usize) {
  let next = (pos.0 + dir, pos.1);
  let cur = wgrid[pos.0][pos.1];
  match cur {
    b'.' => (),
    b'@' => {
      push(wgrid, next, dir);
      wgrid[next.0][next.1] = b'@';
      wgrid[pos.0][pos.1] = b'.';
    }
    b'[' => {
      push(wgrid, next, dir);
      push(wgrid, (next.0, next.1 + 1), dir);
      wgrid[next.0][next.1] = b'[';
      wgrid[pos.0][pos.1] = b'.';
      wgrid[next.0][next.1 + 1] = b']';
      wgrid[pos.0][pos.1 + 1] = b'.';
    }
    b']' => {
      push(wgrid, (pos.0, pos.1 - 1), dir);
    }
    _ => unreachable!(),
  }
}

fn wstep(wgrid: &mut Vec<Vec<u8>>, wpos: (usize, usize), dir: usize) -> bool {
  let next = (wpos.0 + dir, wpos.1);
  match wgrid[next.0][next.1] {
    b'.' => true,
    b'#' => false,
    b'[' => wstep(wgrid, next, dir) && wstep(wgrid, (next.0, next.1 + 1), dir),
    b']' => wstep(wgrid, next, dir) && wstep(wgrid, (next.0, next.1 - 1), dir),
    _ => unreachable!(),
  }
}

fn get_score(grid: &Vec<Vec<u8>>, target: u8) -> usize {
  grid
    .iter()
    .enumerate()
    .map(|(row, line)|
      line
        .iter()
        .enumerate()
        .map(|(col, c)| if *c == target { row * 100 + col } else { 0 })
        .sum::<usize>()
    )
    .sum()
}

#[aoc24::main(15)]
fn main(input: &str) -> (usize, usize) {
  let (_grid, _steps) = input.split_once("\n\n").unwrap();
  let mut grid = _grid
    .split('\n')
    .map(|line| line.as_bytes().to_vec())
    .collect_vec();
  let steps = _steps.replace('\n', "").as_bytes().to_vec();
  let size = grid.len();
  let mut pos = (0..size)
    .cartesian_product(0..size)
    .filter(|(i, j)| grid[*i][*j] == b'@')
    .next()
    .unwrap();

  let mut wgrid = grid
    .iter()
    .map(|line|
      line
        .iter()
        .map(|c| {
          match *c {
            b'.' => vec![b'.', b'.'],
            b'#' => vec![b'#', b'#'],
            b'O' => vec![b'[', b']'],
            b'@' => vec![b'@', b'.'],
            _ => unreachable!(),
          }
        })
        .flat_map(|x| x)
        .collect_vec()
    )
    .collect_vec();
  let mut wpos = (pos.0, 2 * pos.1);

  steps.iter().for_each(|x| {
    let wresult = match x {
      b'^' => {
        if wstep(&mut wgrid, wpos, usize::MAX) {
          push(&mut wgrid, wpos, usize::MAX);
          Some((wpos.0 - 1, wpos.1))
        } else {
          None
        }
      }
      b'v' => {
        if wstep(&mut wgrid, wpos, 1) {
          push(&mut wgrid, wpos, 1);
          Some((wpos.0 + 1, wpos.1))
        } else {
          None
        }
      }
      b'>' => step(&mut wgrid, wpos, (0, 1)),
      b'<' => step(&mut wgrid, wpos, (0, usize::MAX)),
      _ => unreachable!(),
    };
    match wresult {
      None => (),
      Some(next) => wpos = next
    }

    let result = match x {
      b'^' => step(&mut grid, pos, (usize::MAX, 0)),
      b'v' => step(&mut grid, pos, (1, 0)),
      b'>' => step(&mut grid, pos, (0, 1)),
      b'<' => step(&mut grid, pos, (0, usize::MAX)),
      _ => unreachable!(),
    };
    match result {
      None => (),
      Some(next) => {
        pos = next;
      }
    }
  });

  (get_score(&grid, b'O'), get_score(&wgrid, b'['))
}
