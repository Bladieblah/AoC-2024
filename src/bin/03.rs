use regex::Regex;

#[aoc24::main(03)]
fn main(input: &str) -> (usize, usize) {
  let mult_regex = Regex::new(r"(mul|do|don't)\((\d*),?(\d*)\)").unwrap();

  let (_, p1, p2) = mult_regex
    .captures_iter(input)
    .map(|c| c.extract::<3>())
    .fold((1, 0, 0), |acc, (_, [kw, l, r])| {
      match kw {
        "mul" => {
          if l == "" || r == "" {
            return acc;
          }
          let v = l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap();
          (acc.0, acc.1 + v, acc.2 + acc.0 * v)
        }
        "do" => (1, acc.1, acc.2),
        "don't" => (0, acc.1, acc.2),
        _ => unreachable!(),
      }
    });

  (p1, p2)
}
