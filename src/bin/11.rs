use hashbrown::HashMap;

fn _step(num: &String) -> (String, Option<String>) {
  if num == "0" {
    (String::from("1"), None)
  } else {
    let l = num.len();
    if l % 2 == 0 {
      let left = num[..l / 2].to_string();
      let right = num[l / 2..l].trim_start_matches('0').to_string();
      if right == "" {
        (left, Some(String::from("0")))
      } else {
        (left, Some(right))
      }
    } else {
      ((num.parse::<usize>().unwrap() * 2024).to_string(), None)
    }
  }
}

fn step(cache: &mut HashMap<(String, usize), usize>, num: &String, rem: usize) -> usize {
  match cache.get(&(num.clone(), rem)) {
    Some(result) => *result,
    None => {
      let result = if rem == 0 {
        1
      } else {
        let next = _step(num);
        step(cache, &next.0, rem - 1) +
          (match next.1 {
            None => 0,
            Some(val) => step(cache, &val, rem - 1),
          })
      };
      cache.insert((num.clone(), rem), result);
      result
    }
  }
}

#[aoc24::main(11)]
fn main(input: &str) -> (usize, usize) {
  let mut cache = HashMap::<(String, usize), usize>::new();
  input
    .split(' ')
    .map(|s| s.to_string())
    .map(|num| { (step(&mut cache, &num, 25), step(&mut cache, &num, 75)) })
    .fold((0, 0), |acc, v| (acc.0 + v.0, acc.1 + v.1))
}
