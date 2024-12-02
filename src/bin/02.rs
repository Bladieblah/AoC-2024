use itertools::Itertools;

fn is_valid_input(report: &Vec<i32>) -> bool {
  let rising: i32 = if report[1] > report[0] {
    1
  } else if report[1] < report[0] {
    -1
  } else {
    return false;
  };

  for (i, x) in report[..report.len() - 1].into_iter().enumerate() {
    let diff = (report[i + 1] - x) * rising;
    if diff < 1 || diff > 3 {
      return false;
    }
  }

  true
}

fn remove_at(v: &Vec<i32>, i: usize) -> Vec<i32> {
  let mut copy = v.to_vec();
  copy.remove(i);
  copy
}

fn check_dampen(v: &Vec<i32>) -> bool {
  for i in 0..v.len() {
    if is_valid_input(&remove_at(v, i)) {
      return true;
    }
  }

  return false;
}

#[aoc23::main(02)]
fn main(input: &str) -> (usize, usize) {
  let reports = input
    .split('\n')
    .map(|line|
      line
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec()
    )
    .collect_vec();

  let valid = reports.into_iter().map(|report| {
    let v1 = is_valid_input(&report);
    if v1 {
      return (true, true);
    } else {
      return (false, check_dampen(&report));
    }
  });

  valid.fold((0, 0), |(acc1, acc2), (v1, v2)| (acc1 + (v1 as usize), acc2 + (v2 as usize)))
}
