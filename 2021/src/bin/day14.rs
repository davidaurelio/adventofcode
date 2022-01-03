#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::mem;

type InsertionRules = HashMap<(char, char), String>;

fn main() -> io::Result<()> {
  let (polymer, rules) = parse_input(io::stdin().lock())?;

  let mut polymers = (polymer, String::new());

  for _ in 0..10 {
    polymers.1.extend(insert(&rules, &polymers.0));
    mem::swap(&mut polymers.0, &mut polymers.1);
    polymers.1.clear();
  }

  let mut counts = HashMap::new();
  for element in polymers.0.chars() {
    *counts.entry(element).or_insert(0) += 1;
  }

  print_result(1, &counts);

  Ok(())
}

fn print_result(n: usize, counts: &HashMap<char, usize>) {
  let mut ordered_counts: Vec<_> = counts
    .iter()
    .map(|(element, count)| (count, element))
    .collect();
  ordered_counts.sort();

  let first = ordered_counts.first().unwrap();
  let last = ordered_counts.last().unwrap();
  println!(
    "{}: {}, {}: {}. Result #{}: {}",
    first.1,
    first.0,
    last.1,
    last.0,
    n,
    last.0 - first.0
  );
}

fn insert<'a>(
  rules: &'a HashMap<(char, char), String>,
  polymer: &'a str,
) -> impl Iterator<Item = char> + 'a {
  polymer[0..1].chars().chain(
    polymer.chars().zip(polymer[1..].chars()).flat_map(|pair| {
      rules
        .get(&pair)
        .map(String::as_str)
        .unwrap_or("")
        .chars()
        .chain([pair.1].into_iter())
    }),
  )
}

fn parse_input(stdin: io::StdinLock) -> io::Result<(String, InsertionRules)> {
  let mut lines = stdin.lines();

  let polymer = lines.next().unwrap_or_else(|| Err(bad_input("")))?;
  let empty_line = lines.next().unwrap_or_else(|| Err(bad_input("")))?;
  if !empty_line.is_empty() {
    return Err(bad_input(&empty_line));
  }

  lazy_static! {
    static ref MATCHER: Regex =
      Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])").unwrap();
  }
  let mut rules = InsertionRules::new();
  for line in lines {
    let line = line?;
    let captures = value_or_bad_input(MATCHER.captures(&line), &line)?;
    let from = (capture_as_char(&captures, 1), capture_as_char(&captures, 2));
    let to = captures.get(3).unwrap().as_str();
    rules.insert(from, to.to_string());
  }

  Ok((polymer, rules))
}

fn value_or_bad_input<T>(option: Option<T>, input: &str) -> io::Result<T> {
  option.ok_or_else(|| bad_input(input))
}

fn capture_as_char(captures: &regex::Captures, idx: usize) -> char {
  captures
    .get(idx)
    .unwrap()
    .as_str()
    .chars()
    .take(1)
    .last()
    .unwrap()
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}
