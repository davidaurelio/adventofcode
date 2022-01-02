use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

pub fn main() -> io::Result<()> {
  let DotsAndFolds {
    mut dots,
    mut folds,
  } = io::stdin().lock().lines().try_into()?;

  apply_fold(folds.pop_front().unwrap(), &mut dots)?;

  println!(
    "# dots after first fold: {}",
    dots.iter().collect::<HashSet<&Dot>>().len()
  );

  while !folds.is_empty() {
    apply_fold(folds.pop_front().unwrap(), &mut dots)?;
  }

  println!("\nPart 2:");
  print_dots(dots, io::stdout().lock())?;

  Ok(())
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Dot {
  pub x: u32,
  pub y: u32,
}

#[derive(Clone, Copy, Debug)]
enum Fold {
  X(u32),
  Y(u32),
}

#[derive(Clone, Debug, Default)]
struct DotsAndFolds {
  pub dots: Vec<Dot>,
  pub folds: VecDeque<Fold>,
}

impl<T> TryFrom<io::Lines<T>> for DotsAndFolds
where
  T: io::BufRead,
{
  type Error = io::Error;

  fn try_from(mut lines: io::Lines<T>) -> io::Result<DotsAndFolds> {
    let mut dots_and_folds = DotsAndFolds::default();

    for result in lines.by_ref() {
      let line = result?;
      if line.is_empty() {
        break;
      }
      dots_and_folds.dots.push(Dot::try_from(line.as_str())?);
    }

    for result in lines {
      let line = result?;
      dots_and_folds
        .folds
        .push_back(Fold::try_from(line.as_str())?);
    }

    Ok(dots_and_folds)
  }
}

impl TryFrom<&str> for Dot {
  type Error = io::Error;
  fn try_from(line: &str) -> io::Result<Dot> {
    let (x, y) = line.split_once(',').ok_or_else(|| bad_input(line))?;
    Ok(Dot {
      x: u32::from_str(x).map_err(|_| bad_input(line))?,
      y: u32::from_str(y).map_err(|_| bad_input(line))?,
    })
  }
}

impl std::fmt::Display for Dot {
  fn fmt(
    &self,
    fmt: &mut std::fmt::Formatter<'_>,
  ) -> Result<(), std::fmt::Error> {
    fmt.write_fmt(format_args!("{},{}", self.x, self.y))
  }
}

impl Ord for Dot {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match self.y.cmp(&other.y) {
      std::cmp::Ordering::Equal => self.x.cmp(&other.x),
      ordering => ordering,
    }
  }
}

impl PartialOrd for Dot {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl TryFrom<&str> for Fold {
  type Error = io::Error;
  fn try_from(line: &str) -> io::Result<Fold> {
    let (m, n) = line.split_once('=').ok_or_else(|| bad_input(line))?;
    let n = u32::from_str(n).map_err(|_| bad_input(line))?;
    if m == "fold along x" {
      Ok(Fold::X(n))
    } else if m == "fold along y" {
      Ok(Fold::Y(n))
    } else {
      Err(bad_input(line))
    }
  }
}

fn apply_fold(fold: Fold, dots: &mut Vec<Dot>) -> io::Result<()> {
  match fold {
    Fold::X(x) => {
      let mut i = 0;
      while i < dots.len() {
        let dot = &mut dots[i];
        if dot.x == x {
          dots.swap_remove(i);
        } else {
          i += 1;
          if dot.x > x {
            dot.x = x.checked_sub(dot.x - x).ok_or_else(|| bad_dot(dot))?
          }
        }
      }
    }
    Fold::Y(y) => {
      let mut i = 0;
      while i < dots.len() {
        let dot = &mut dots[i];
        if dot.y == y {
          dots.swap_remove(i);
        } else {
          i += 1;
          if dot.y > y {
            dot.y = y.checked_sub(dot.y - y).ok_or_else(|| bad_dot(dot))?
          }
        }
      }
    }
  };

  Ok(())
}

fn print_dots<W>(mut dots: Vec<Dot>, write: W) -> io::Result<()>
where
  W: io::Write,
{
  let mut write = io::BufWriter::new(write);
  dots.sort();
  dots.dedup();

  let mut current = Dot { x: 0, y: 0 };

  for dot in dots {
    if current.y < dot.y {
      for _ in current.y..dot.y {
        write.write_all(b"\n")?;
      }
      current = Dot { x: 0, y: dot.y };
    }
    if current.x < dot.x {
      for _ in current.x..dot.x {
        write.write_all(b" ")?;
      }
    }
    write.write_all(b"#")?;
    current.x = dot.x + 1;
  }
  write.write_all(b"\n")?;
  write.flush()
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}

fn bad_dot(dot: &Dot) -> io::Error {
  bad_input(&dot.to_string())
}
