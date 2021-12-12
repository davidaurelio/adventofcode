#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::ops::RangeInclusive;

pub fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let stdin_lock = stdin.lock();

  let n = &read_coords(stdin_lock)?
    .values()
    .filter(|&&v| v > 1)
    .count();
  println!("{:?} overlaps", n);

  Ok(())
}

type Point = [u32; 2];
type Vector = [Point; 2];

fn read_coords(stdin: io::StdinLock) -> io::Result<HashMap<Point, u32>> {
  let mut coords = HashMap::new();
  stdin
    .lines()
    .map(|r| r.and_then(|l| parse_line(&l)))
    .filter_map(|r| r.map(as_straight_line).transpose())
    .try_for_each(|r| match r {
      Err(e) => Err(e),
      Ok([xs, ys]) => {
        for x in xs {
          for y in Clone::clone(&ys) {
            *coords.entry([x, y]).or_insert(0) += 1;
          }
        }
        Ok(())
      }
    })?;
  Ok(coords)
}

fn parse_line(line: &str) -> io::Result<Vector> {
  let (x1, y1, x2, y2) =
    scan_fmt!(line, "{d},{d} -> {d},{d}{e}", u32, u32, u32, u32)
      .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, line))?;

  Ok([[x1, y1], [x2, y2]])
}

fn as_straight_line(
  [[x1, y1], [x2, y2]]: Vector,
) -> Option<[RangeInclusive<u32>; 2]> {
  if (x1 == x2) || (y1 == y2) {
    Some([make_range(x1, x2), make_range(y1, y2)])
  } else {
    None
  }
}

fn make_range(a: u32, b: u32) -> RangeInclusive<u32> {
  a.min(b)..=a.max(b)
}
