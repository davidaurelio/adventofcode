#[macro_use]
extern crate scan_fmt;

use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::iter::repeat;
use std::mem::replace;
use std::ops;

pub fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let stdin_lock = stdin.lock();

  let n = &read_coords(stdin_lock)?.count_intersections();
  println!("{:?} overlaps", n);

  Ok(())
}

type Point = [u32; 2];
type Vector = [Point; 2];

fn read_coords(stdin: io::StdinLock) -> io::Result<IntersectionCounts> {
  let mut counts = IntersectionCounts::new();
  stdin
    .lines()
    .map(|r| r.and_then(|l| parse_line(&l)))
    .try_for_each(|r| r.map(|line| add_line(&mut counts, line)))?;
  Ok(counts)
}

struct IntersectionCounts {
  counts: HashMap<Point, u32>,
}

impl IntersectionCounts {
  pub fn new() -> Self {
    Self {
      counts: HashMap::new(),
    }
  }

  pub fn add_point(&mut self, point: Point) {
    *self.counts.entry(point).or_insert(0) += 1
  }

  pub fn add_points<T>(&mut self, points: T)
  where
    T: Iterator<Item = (u32, u32)>,
  {
    for (x, y) in points {
      self.add_point([x, y]);
    }
  }

  pub fn count_intersections(&self) -> usize {
    self.counts.values().filter(|&&v| v > 1).count()
  }
}

fn parse_line(line: &str) -> io::Result<Vector> {
  let (x1, y1, x2, y2) =
    scan_fmt!(line, "{d},{d} -> {d},{d}{e}", u32, u32, u32, u32)
      .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, line))?;

  Ok([[x1, y1], [x2, y2]])
}

fn add_line(counts: &mut IntersectionCounts, [[x1, y1], [x2, y2]]: Vector) {
  if x1 == x2 {
    if y1 == y2 {
      counts.add_point([x1, y1])
    } else {
      counts.add_points(repeat(x1).zip(range(y1, y2)))
    }
  } else if y1 == y2 {
    counts.add_points(range(x1, x2).zip(repeat(y1)))
  } else {
    counts.add_points(range(x1, x2).zip(range(y1, y2)))
  }
}

struct AnyDirRange<Idx> {
  pub start: Idx,
  pub end: Idx,
}

impl<Idx> Iterator for AnyDirRange<Idx>
where
  Idx: PartialOrd
    + Copy
    + From<u8>
    + ops::Add<Output = Idx>
    + ops::Sub<Output = Idx>,
{
  type Item = Idx;
  fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    let next = match self.start.partial_cmp(&self.end) {
      Some(Ordering::Less) => self.start + 1.into(),
      Some(Ordering::Greater) => self.start - 1.into(),
      _ => return None,
    };
    Some(replace(&mut self.start, next))
  }
}

fn range(start: u32, end: u32) -> AnyDirRange<u32> {
  AnyDirRange {
    start,
    end: if start > end { end - 1 } else { end + 1 },
  }
}
