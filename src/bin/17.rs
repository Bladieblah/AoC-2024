use itertools::Itertools;

fn run(_regs: (usize,usize,usize), program: &Vec<usize>, size: usize, check: bool) -> Vec<usize> {
  let mut regs = _regs.clone();
  let combo = |x: usize, cur: (usize,usize,usize)| {
    match x {
      0..=3 => x,
      4 => cur.0,
      5 => cur.1,
      6 => cur.2,
      7 => panic!(),
      _ => unreachable!()
    }
  };
  let mut ptr = 0_usize;
  let mut output = Vec::<usize>::new();
  while ptr < size {
    let cur = regs.clone();
    let operand = program[ptr + 1];
    match program[ptr] {
      0 => regs.0 = cur.0 / 2_usize.pow(combo(operand, cur) as u32),
      1 => regs.1 = cur.1 ^ operand,
      2 => regs.1 = combo(operand, cur) % 8,
      3 => if regs.0 > 0 { ptr = operand - 2 },
      4 => regs.1 = cur.1 ^ cur.2,
      5 => {
        output.push(combo(operand, cur) % 8);
        if check {
          let outsize = output.len() - 1;
          if output[outsize] != program[outsize] {
            break;
          }
        }
      },
      6 => regs.1 = cur.0 / 2_usize.pow(combo(operand, cur) as u32),
      7 => regs.2 = cur.0 / 2_usize.pow(combo(operand, cur) as u32),
      _ => unreachable!()
    };
    ptr = ptr + 2;
  }
  output
}

fn find_sol(program: &Vec<usize>, size: usize, cur: usize, stepsize: usize, maxlen: usize, curpow: usize) -> Option<usize> {
  if curpow as i64 - maxlen as i64 > 1 {
    return None;
  }
  let mut nxt = cur + stepsize;
  while nxt < 11 * stepsize {
    let res = run((nxt, 0, 0), program, size, true);
    if res.len() == size && res[size-1] == program[size-1] {
      return Some(nxt);
    } else if res.len() >= maxlen {
      match find_sol(program, size, nxt, stepsize * 8, res.len(), curpow + 1) {
        Some(value) => return Some(value),
        None => ()
      }
    }
    nxt += stepsize;
  };
  None
}

#[aoc24::main(17)]
fn main(input: &str) -> (String, usize) {
  let (_regs, _program) = input.split_once("\n\n").unwrap();
  let regs = _regs
    .split('\n')
    .map(|line| line.split_once(": ").unwrap().1.parse::<usize>().unwrap())
    .collect_tuple::<(_, _, _)>()
    .unwrap();
  let program = _program.split_once(' ').unwrap().1.split(',').map(|x| x.parse::<usize>().unwrap()).collect_vec();
  let size = program.len();
  let p1 = run(regs, &program, size, false);
  let res = find_sol(&program, size, 0, 1, 0, 0);
  let p2 = if res.is_some() { res.unwrap() } else { 0 };
  (format!("{:?}", p1).replace(' ', ""), p2)
}
