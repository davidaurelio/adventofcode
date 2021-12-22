use std::io;
use std::io::BufRead;
use std::ops::Index;

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let height_map: HeightMap = stdin.lock().lines().try_into()?;

  let risk = height_map
    .coords()
    .filter_map(|coord| {
      let height = height_map[coord];
      if height_map
        .neighbours(coord)
        .all(|neighbour| height_map[neighbour] > height)
      {
        Some(height)
      } else {
        None
      }
    })
    .map(risk_level)
    .fold(0u32, |sum, risk| sum + risk as u32);

  println!("Sum of risks {}", risk);

  Ok(())
}

struct HeightMap {
  width: usize,
  heights: Vec<u8>,
}

impl HeightMap {
  pub fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
    let rows: usize = self.heights.len() / self.width;
    (0..rows).flat_map(|row| (0..self.width).map(move |col| (row, col)))
  }

  pub fn neighbours(
    &self,
    (row, col): (usize, usize),
  ) -> impl Iterator<Item = (usize, usize)> {
    let width = self.width;
    let height = self.heights.len() / width;
    let mut neighbours = [None; 4];

    if row > 0 {
      neighbours[0] = Some((row - 1, col));
    }
    if col + 1 < width {
      neighbours[1] = Some((row, col + 1));
    }
    if row + 1 < height {
      neighbours[2] = Some((row + 1, col));
    }
    if col > 0 {
      neighbours[3] = Some((row, col - 1))
    }

    neighbours.into_iter().flatten()
  }
}

impl Index<(usize, usize)> for HeightMap {
  type Output = u8;
  fn index(&self, (row, col): (usize, usize)) -> &u8 {
    &self.heights[row * self.width + col]
  }
}

impl<T: BufRead> TryFrom<io::Lines<T>> for HeightMap {
  type Error = io::Error;

  fn try_from(lines: io::Lines<T>) -> io::Result<Self> {
    let mut lines = lines.peekable();
    let width = lines
      .peek()
      .and_then(|r| r.as_ref().map(|line| line.len()).ok())
      .unwrap_or(0);

    let mut heights = vec![];
    for line_result in lines {
      match line_result {
        Err(err) => return Err(err),
        Ok(line) => {
          let mut digit_result = Ok(());
          let digits =
            line.chars().map(parse_digit).map_while(
              |digit_res| match digit_res {
                Ok(digit) => Some(digit),
                Err(err) => {
                  digit_result = Err(err);
                  None
                }
              },
            );
          heights.extend(digits);
          digit_result?;
        }
      }
    }

    Ok(Self { heights, width })
  }
}

fn risk_level(height: u8) -> u8 {
  height + 1
}

fn parse_digit(chr: char) -> io::Result<u8> {
  match chr {
    '0'..='9' => Ok(chr as u8 - b'0'),
    _ => Err(bad_input(&chr.to_string())),
  }
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}
