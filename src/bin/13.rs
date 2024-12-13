use regex::Regex;

fn solve(det: i64, mat: ((i64,i64),(i64,i64)), t: (i64,i64)) -> usize {
  let _a = t.0 * mat.1.1 - t.1 * mat.0.1;
  if _a % det != 0 { return 0; }
  let _b = t.1 * mat.0.0 - t.0 * mat.1.0;
  if _b % det != 0 { return 0; }
  let (a, b) = (_a / det, _b / det);
  (3 * a + b) as usize
}

#[aoc24::main(13)]
fn main(input: &str) -> (usize, usize) {
  let re = Regex::new(r"X.(\d+), Y.(\d+)").unwrap();
  input.split("\n\n").map(|m| {
    let mut lines = m.splitn(3, '\n');
    let (_, [a_x, a_y]) = re.captures(lines.next().unwrap()).unwrap().extract::<2>();
    let (_, [b_x, b_y]) = re.captures(lines.next().unwrap()).unwrap().extract::<2>();
    let (_, [t_x, t_y]) = re.captures(lines.next().unwrap()).unwrap().extract::<2>();
    let mat = (
      (a_x.parse::<i64>().unwrap(), b_x.parse::<i64>().unwrap()),
      (a_y.parse::<i64>().unwrap(), b_y.parse::<i64>().unwrap()),
    );
    let det: i64 = mat.0.0 * mat.1.1 - mat.1.0 * mat.0.1;
    if det == 0 { return (0, 0); }
    let t = (t_x.parse::<i64>().unwrap(), t_y.parse::<i64>().unwrap());
    (solve(det, mat, t), solve(det, mat, (t.0 + 10000000000000, t.1 + 10000000000000)))
  })
  .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}
