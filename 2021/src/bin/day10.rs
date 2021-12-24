#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::BufRead;

lazy_static! {
  static ref CURRUPT_SCORES: [i32; 128] = {
    let mut scores = [0; 128];
    scores[')' as usize] = 3;
    scores[']' as usize] = 57;
    scores['}' as usize] = 1197;
    scores['>' as usize] = 25137;
    scores
  };
  static ref MISSING_SCORES: [i64; 128] = {
    let mut scores = [0; 128];
    scores[')' as usize] = 1;
    scores[']' as usize] = 2;
    scores['}' as usize] = 3;
    scores['>' as usize] = 4;
    scores
  };
  static ref CLOSING: [char; 128] = {
    let mut closing = ['\0'; 128];
    closing['(' as usize] = ')';
    closing['[' as usize] = ']';
    closing['{' as usize] = '}';
    closing['<' as usize] = '>';
    closing
  };
}

fn main() -> io::Result<()> {
  let lines = {
    let stdin = io::stdin();
    stdin.lock().lines().collect::<io::Result<Vec<String>>>()
  }?;

  println!("Corrupt score: {}", corrupt_score(&lines));
  println!("Missing score: {}", missing_score(&lines));

  Ok(())
}

fn corrupt_score(lines: &[String]) -> i32 {
  lines
    .iter()
    .filter_map(|line| corrupt_closing_delimiter(line))
    .fold(0, |score, corrupt| CURRUPT_SCORES[corrupt as usize] + score)
}

fn corrupt_closing_delimiter(line: &str) -> Option<char> {
  let mut expected = vec![];
  for c in line.chars() {
    match c {
      '(' | '[' | '{' | '<' => expected.push(CLOSING[c as usize]),
      ')' | ']' | '}' | '>' => {
        if expected.last() == Some(&c) {
          expected.pop();
        } else {
          return Some(c);
        }
      }
      _ => (),
    };
  }
  None
}

fn missing_score(lines: &[String]) -> i64 {
  let mut line_scores = lines
    .iter()
    .filter_map(|line| missing_delimiters(line))
    .map(|line| {
      line.iter().rev().fold(0, |score, &delim| {
        score * 5 + MISSING_SCORES[delim as usize]
      })
    })
    .collect::<Vec<_>>();
  let mid = line_scores.len() / 2;
  *line_scores[..].select_nth_unstable(mid).1
}

fn missing_delimiters(line: &str) -> Option<Vec<char>> {
  let mut expected = vec![];
  for c in line.chars() {
    match c {
      '(' | '[' | '{' | '<' => expected.push(CLOSING[c as usize]),
      ')' | ']' | '}' | '>' => {
        if expected.last() == Some(&c) {
          expected.pop();
        } else {
          return None;
        }
      }
      _ => (),
    };
  }
  Some(expected)
}
