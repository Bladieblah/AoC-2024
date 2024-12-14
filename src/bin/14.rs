use regex::Regex;

fn step(pos: &mut Vec<(i32, i32)>, vel: &Vec<(i32, i32)>, width: i32, height: i32) -> (f32, f32) {
  vel
    .iter()
    .enumerate()
    .for_each(|(i, v)| {
      pos[i].0 = (pos[i].0 + v.0).rem_euclid(width);
      pos[i].1 = (pos[i].1 + v.1).rem_euclid(height);
    });

  let mut mean = pos.iter().fold((0.0, 0.0), |acc, p| (acc.0 + (p.0 as f32), acc.1 + (p.1 as f32)));
  mean = (mean.0 / 500.0, mean.1 / 500.0);
  let var = pos
    .iter()
    .fold((0.0, 0.0), |acc, p| (
      acc.0 + ((p.0 as f32) - mean.0).powf(2.0),
      acc.1 + ((p.1 as f32) - mean.1).powf(2.0)
    ));
  (var.0 / 500.0, var.1 / 500.0)
}

#[aoc24::main(14)]
fn main(input: &str) -> (usize, usize) {
  let width = 101;
  let height = 103;
  let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
  let mut quad_counts = vec![0; 4];
  let (mut pos, vel): (Vec<_>, Vec<_>) = input
    .split('\n')
    .map(|line| {
      let [p_x, p_y, v_x, v_y] = re
        .captures(line)
        .unwrap()
        .extract::<4>()
        .1.map(|x| x.parse::<i32>().unwrap());
      let x_end = (p_x + 100 * v_x).rem_euclid(width);
      let y_end = (p_y + 100 * v_y).rem_euclid(height);
      match (x_end, y_end) {
        (0..=49, 0..=50) => quad_counts[0] = quad_counts[0] + 1,
        (0..=49, 52..=103) => quad_counts[1] = quad_counts[1] + 1,
        (51..=101, 0..=50) => quad_counts[2] = quad_counts[2] + 1,
        (51..=101, 52..=103) => quad_counts[3] = quad_counts[3] + 1,
        _ => ()
      };
      ((p_x, p_y), (v_x, v_y))
    })
    .unzip();

  let mut start = (0, 0);
  for i in 1..=103 {
    // x and y variance both have cycles of length width / height respectively, find the starting points
    let var = step(&mut pos, &vel, width, height);
    if var.0 < 400.0 { start.0 = i; }
    if var.1 < 400.0 { start.1 = i; }
  }

  // Solve x0 + k_x * width == y0 + k_y * height for integer k_x, k_y
  let mut diff = start.0 - start.1;
  let mut k = 0;
  while diff < 0 || diff % 2 != 0 {
    diff += width;
    k += 1;
  };

  (quad_counts.into_iter().fold(1, |acc, x| acc * x), (start.0 + (k + diff / 2) * width) as usize)
}
