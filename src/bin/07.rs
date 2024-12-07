use itertools::Itertools;
use std::ops::{ Add, Mul };

fn solve(target: usize, cur: usize, inputs: &Vec<usize>, ops: &Vec<fn(usize, usize) -> usize>) -> usize {
  if cur > target { return 0; }
  match inputs.len() {
    0 => if cur == target { cur } else { 0 }
    _ => {
      let remainder = Vec::from_iter(inputs[1..].iter().cloned());
      for op in ops {
        let res = solve(target, op(cur, inputs[0]), &remainder, ops);
        if res != 0 { return res; }
      }
      0
    }
  }
}

fn concat(a: usize, b: usize) -> usize {
  format!("{}{}", a, b).parse::<usize>().unwrap()
}

#[aoc24::main(07)]
fn main(input: &str) -> (usize, usize) {
  let ops1: Vec<fn(usize, usize) -> usize> = vec![<usize as Add>::add, <usize as Mul>::mul];
  let ops2: Vec<fn(usize, usize) -> usize> = vec![<usize as Add>::add, <usize as Mul>::mul, concat];
  let p = input
    .split('\n')
    .map(|line| line.split_once(": ").unwrap())
    .map(|(out, inputs)| (
      out.parse::<usize>().unwrap(),
      inputs
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec(),
    ))
    .map(|(target, inputs)| {
      let remainder = Vec::from_iter(inputs[1..].iter().cloned());
      let s1 = solve(target, inputs[0], &remainder, &ops1);
      if s1 > 0 {
        (s1, 0)
      } else {
        (0, solve(target, inputs[0], &remainder, &ops2))
      }
    })
    .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1));

  (p.0, p.0 + p.1)
}
