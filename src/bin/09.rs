use itertools::Itertools;

fn part1(files: Vec<usize>) -> usize {
  let mut filler = files.clone();

  let mut pos = 0;
  let mut p1: usize = 0;
  let mut empty = false;
  let mut cur_id: usize = 0;
  let mut endpoint = files.len() - 1;
  let mut end_id: usize = endpoint / 2;

  for _x in files {
    let mut x = _x;
    if cur_id >= end_id {
      p1 = p1 + cur_id * (pos..pos + filler[endpoint]).sum::<usize>();
      break;
    }
    if empty {
      loop {
        if filler[endpoint] >= x {
          p1 = p1 + end_id * (pos..pos + x).sum::<usize>();
          pos = pos + x;
          filler[endpoint] = filler[endpoint] - x;
          break;
        } else {
          p1 = p1 + end_id * (pos..pos + filler[endpoint]).sum::<usize>();
          pos = pos + filler[endpoint];
          x = x - filler[endpoint];
          filler[endpoint] = 0;
          endpoint -= 2;
          end_id -= 1;
        }
      }
    } else {
      p1 = p1 + cur_id * (pos..pos + x).sum::<usize>();
      pos = pos + x;
      cur_id += 1;
    }
    empty = !empty;
  }
  p1
}

fn part2(_files: Vec<usize>) -> usize {
  let mut files: Vec<(usize, usize, usize)> = Vec::with_capacity(_files.len());
  let mut space: Vec<(usize, usize)> = Vec::with_capacity(_files.len());
  let mut pos = 0;

  for (i, x) in _files.iter().enumerate() {
    if i % 2 == 0 {
      files.push((pos, *x, i / 2));
    } else {
      space.push((pos, *x));
    }
    pos += *x;
  }

  for (start, size, _) in &mut files.iter_mut().rev() {
    for (pos, cap) in &mut space.iter_mut() {
      if pos > start {
        break;
      }
      if *cap >= *size {
        *start = *pos;
        *cap = *cap - *size;
        *pos = *pos + *size;
        break;
      }
    }
  }

  files.into_iter().fold(0, |acc, (start, size, id)| acc + id * (start..start + size).sum::<usize>())
}

#[aoc24::main(09)]
fn main(input: &str) -> (usize, usize) {
  let files = input
    .as_bytes()
    .iter()
    .map(|x| (x - b'0') as usize)
    .collect_vec();
  let p1 = part1(files.clone());
  let p2 = part2(files);
  (p1, p2)
}
