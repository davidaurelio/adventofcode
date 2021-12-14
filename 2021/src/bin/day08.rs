#[macro_use]
extern crate lazy_static;
extern crate partition;
extern crate regex;

use partition::partition;
use regex::Regex;

use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
use std::ops::BitOr;
use std::ops::Range;

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let results = parse_input(stdin.lock())
    .map(|r| r.and_then(solve_display))
    .collect::<io::Result<Vec<_>>>()?;

  let mut counts = [0usize; 10];
  for digits in &results {
    for &digit in digits {
      counts[digit as usize] += 1;
    }
  }

  println!(
    "Number of 1, 4, 7, 8s: {:?}",
    counts[1] + counts[4] + counts[7] + counts[8]
  );

  println!(
    "Sum of all displays: {:?}",
    results
      .iter()
      .map(|[m, c, x, i]| m * 1000 + c * 100 + x * 10 + i)
      .sum::<u32>()
  );

  Ok(())
}

fn solve_display(mut display: Display) -> io::Result<[u32; 4]> {
  let mut missing = segment::ALL_SEGMENTS;
  let mut candidates = [0; 7];
  display.digits.sort_by(cmp_display_ranges);
  let mut digits_iter = display.digits.iter();

  let mut next = || next_digit(&mut digits_iter, &display.line);

  let one = next()?;
  missing &= !one;
  candidates[2] = one;
  candidates[5] = one;

  let seven = next()?;
  missing &= !seven;
  candidates[0] = seven & !one;

  let four = next()?;
  missing &= !four;
  candidates[1] = four & !one;
  candidates[3] = four & !one;

  let check = |condition: bool, message: &str| {
    if condition {
      Ok(())
    } else {
      Err(bad_input(&format!("{} {}", &display.line, message)))
    }
  };

  let [two, three, _] = {
    let mut two_three_five = [next()?, next()?, next()?];
    partition(&mut two_three_five, |n| (n & four).count_ones() == 2); // 2 is  first
    partition(&mut two_three_five[1..], |n| (n & one).count_ones() == 2); // 3 is next
    two_three_five
  };

  check(
    candidates[0] & two == candidates[0],
    "two does not contain segment 0",
  )?;
  missing &= !two;
  candidates[1] &= !two;
  candidates[2] &= two & !candidates[0];
  candidates[3] &= two & !(one | seven);
  candidates[4] = two & !(one | seven | four);
  candidates[5] &= !two;
  candidates[6] = two & !(one | seven | four);

  check(missing == 0, "not all segments seen")?;

  candidates[4] &= !three;
  candidates[5] &= three & !two;
  candidates[6] &= three;

  check(
    candidates.iter().all(|c| c.count_ones() == 1),
    "segments have more than one candidate",
  )?;
  check(
    candidates.iter().fold(0, BitOr::bitor) == segment::ALL_SEGMENTS,
    "not all segments have a unique value",
  )?;

  Ok(display.outputs.map(|o| {
    let mut segments: u32 = 0;
    for segment_char in display.line[o].chars() {
      segments |= find_mapping(segment_char, &candidates);
    }
    digit::with_segments(segments)
  }))
}

fn parse_input(
  stdin: io::StdinLock,
) -> impl Iterator<Item = io::Result<Display>> + '_ {
  stdin.lines().map(|r| r.and_then(Display::new))
}

fn next_digit(
  digits_iter: &mut std::slice::Iter<Range<usize>>,
  line: &str,
) -> io::Result<u32> {
  match digits_iter.next() {
    Some(range) => Ok(segment::parse(&line[range.clone()])),
    None => Err(bad_input(line)),
  }
}

#[derive(Clone, Debug)]
struct Display {
  line: String,
  digits: [Range<usize>; 10],
  outputs: [Range<usize>; 4],
}

mod segment {
  use std::ops::BitOr;

  pub const A: u32 = 1 << 0;
  pub const B: u32 = 1 << 1;
  pub const C: u32 = 1 << 2;
  pub const D: u32 = 1 << 3;
  pub const E: u32 = 1 << 4;
  pub const F: u32 = 1 << 5;
  pub const G: u32 = 1 << 6;

  pub const ALL_SEGMENTS: u32 = 0b1111111;
  pub fn parse(segments: &str) -> u32 {
    segments
      .chars()
      .map(|c| match c {
        'a'..='g' => 1 << (c as u32 - 'a' as u32),
        _ => 0,
      })
      .reduce(BitOr::bitor)
      .unwrap_or(0)
  }
}

mod digit {
  use super::segment::{A, B, C, D, E, F, G};

  static DIGIT_SEGMENTS: [(u32, u32); 10] = [
    (C | F, 1),
    (A | C | D | E | G, 2),
    (A | C | D | F | G, 3),
    (B | C | D | F, 4),
    (A | B | D | F | G, 5),
    (A | B | D | E | F | G, 6),
    (A | C | F, 7),
    (A | B | C | D | E | F | G, 8),
    (A | B | C | D | F | G, 9),
    (A | B | C | E | F | G, 0),
  ];

  pub fn with_segments(segments: u32) -> u32 {
    DIGIT_SEGMENTS
      .iter()
      .find(|(bits, _)| *bits == segments)
      .map_or(u32::max_value(), |(_, n)| *n)
  }
}
impl Display {
  pub fn new(line: String) -> io::Result<Display> {
    let pipe_index = if let Some(pipe_index) = line.find('|') {
      pipe_index
    } else {
      return Err(bad_input(&line));
    };

    let digits: [Range<usize>; 10] =
      if let Some(digits) = display_segments(&line[..pipe_index]) {
        digits
      } else {
        return Err(bad_input(&line));
      };

    let outputs: [Range<usize>; 4] =
      if let Some(mut outputs) = display_segments(&line[pipe_index..]) {
        for o in &mut outputs {
          o.start += pipe_index;
          o.end += pipe_index;
        }
        outputs
      } else {
        return Err(bad_input(&line));
      };

    Ok(Self {
      line,
      digits,
      outputs,
    })
  }
}

fn display_segments<const N: usize>(x: &str) -> Option<[Range<usize>; N]> {
  const INIT: Range<usize> = 0..0;
  lazy_static! {
    static ref MATCHER: Regex = Regex::new(r"[a-g]+").unwrap();
  }

  let mut result = [INIT; N];
  let mut matches = MATCHER.find_iter(x);
  for x in result.iter_mut() {
    match matches.next() {
      Some(m) => *x = m.range(),
      None => return None,
    };
  }

  Some(result)
}

fn find_mapping(chr: char, mappings: &[u32; 7]) -> u32 {
  let x = match chr {
    'a'..='g' => 1u32 << (chr as u32 - 'a' as u32),
    _ => return 0,
  };

  mappings
    .iter()
    .enumerate()
    .find(|(_, &c)| c == x)
    .map_or(0, |(i, _)| 1 << i)
}

fn cmp_display_ranges(x: &Range<usize>, y: &Range<usize>) -> Ordering {
  let a = x.len();
  let b = y.len();

  if a == b {
    Ordering::Equal
  } else if b == 6 {
    Ordering::Less
  } else if a == 6 {
    Ordering::Greater
  } else if a < b {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}
