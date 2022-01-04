#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::mem;

type ElementCount = HashMap<u8, usize>;
type Pair = (u8, u8);
type PairCounts = HashMap<Pair, usize>;
type InsertionRules = HashMap<Pair, [Pair; 2]>;

fn main() -> io::Result<()> {
  let (pair_counts, rules, start_counts) = parse_input(io::stdin().lock())?;
  let mut pair_counts = (pair_counts, PairCounts::new());

  print_result(1, steps(10, &mut pair_counts, &rules), start_counts.clone());
  print_result(2, steps(30, &mut pair_counts, &rules), start_counts);
  Ok(())
}

fn steps<'a>(
  n: usize,
  pair_counts: &'a mut (PairCounts, PairCounts),
  rules: &InsertionRules,
) -> &'a PairCounts {
  let first = &mut pair_counts.0;
  let second = &mut pair_counts.1;

  for _ in 0..n {
    for (pair, count) in transform(rules, first) {
      *second.entry(pair).or_insert(0) += count;
    }
    first.clear();
    mem::swap(first, second);
  }
  first
}

fn transform<'a>(
  rules: &'a InsertionRules,
  pair_counts: &'a PairCounts,
) -> impl Iterator<Item = (Pair, usize)> + 'a {
  pair_counts.iter().flat_map(|(pair, count)| {
    let pairs: [Pair; 2] = rules[pair];
    [(pairs[0], *count), (pairs[1], *count)]
  })
}

fn print_result(
  n: usize,
  pair_counts: &PairCounts,
  mut element_counts: ElementCount,
) {
  for (pair, count) in pair_counts {
    *element_counts.entry(pair.0).or_insert(0) += count;
    *element_counts.entry(pair.1).or_insert(0) += count;
  }

  let [min, max] = element_counts.iter().fold(
    [('?', usize::max_value()), ('?', 0)],
    |[min, max], (element, count)| {
      let count = count / 2;
      [
        order_counts(min, (*element as char, count), Ordering::Less),
        order_counts(max, (*element as char, count), Ordering::Greater),
      ]
    },
  );

  println!(
    "{}: {}, {}: {}. Result #{}: {}",
    min.0,
    min.1,
    max.0,
    max.1,
    n,
    max.1 - min.1
  );
}

fn order_counts(
  a: (char, usize),
  b: (char, usize),
  ordering: Ordering,
) -> (char, usize) {
  if a.1.cmp(&b.1) == ordering {
    a
  } else {
    b
  }
}

fn parse_input(
  stdin: io::StdinLock,
) -> io::Result<(PairCounts, InsertionRules, ElementCount)> {
  let mut lines = stdin.lines();

  let polymer = lines
    .next()
    .unwrap_or_else(|| Err(bad_input("")))?
    .into_bytes();
  let empty_line = lines.next().unwrap_or_else(|| Err(bad_input("")))?;
  if !empty_line.is_empty() {
    return Err(bad_input(&empty_line));
  }

  lazy_static! {
    static ref MATCHER: Regex =
      Regex::new(r"^([A-Z][A-Z]) -> ([A-Z])").unwrap();
  }
  let mut seen = HashSet::new();
  let mut rules = InsertionRules::new();
  for line in lines {
    let line = line?;
    let captures = value_or_bad_input(MATCHER.captures(&line), &line)?;
    let from = capture_as_pair(captures.get(1).unwrap());
    let to = captures.get(2).unwrap().as_str().as_bytes()[0];
    seen.extend([from.0, from.1, to]);
    rules.insert(from, [(from.0, to), (to, from.1)]);
  }

  seen.extend(&polymer);

  if rules.len() == seen.len().pow(2) {
    let mut counts = PairCounts::new();
    for pair in polymer.iter().copied().zip(polymer[1..].iter().copied()) {
      *counts.entry(pair).or_insert(0) += 1;
    }
    Ok((
      counts,
      rules,
      [(polymer[0], 1), (polymer[polymer.len() - 1], 1)].into(),
    ))
  } else {
    Err(bad_input(""))
  }
}

fn value_or_bad_input<T>(option: Option<T>, input: &str) -> io::Result<T> {
  option.ok_or_else(|| bad_input(input))
}

fn capture_as_pair(re_match: regex::Match) -> Pair {
  let bytes = re_match.as_str().as_bytes();
  (bytes[0], bytes[1])
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}
