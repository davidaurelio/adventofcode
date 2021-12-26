use std::fmt;
use std::fmt::Write;
use std::io;
use std::io::Read;
use std::ops;

#[derive(Copy, Clone, Default, Debug)]
struct Coord {
  row: usize,
  col: usize,
}

struct Map {
  cells: [u8; 100],
}

fn main() -> io::Result<()> {
  let mut map = Map::from_iter(io::stdin().lock().bytes())?;

  let mut flashes = 0;
  for _ in 0..100 {
    flashes += run_step(&mut map);
  }
  println!("# flashes: {}", flashes);

  Ok(())
}

const FLASH_THRESHOLD: u8 = 10;

fn run_step(map: &mut Map) -> usize {
  let mut flashes: Vec<_> = map
    .enumerate_mut()
    .filter_map(|(coord, value)| {
      *value += 1;
      match value {
        FLASH_THRESHOLD.. => Some(coord),
        _ => None,
      }
    })
    .collect();

  let mut neighbours_dest = [Coord::default(); 8];

  for i in 0.. {
    if i >= flashes.len() {
      break;
    }

    for neighbour in neighbours(&flashes[i], &mut neighbours_dest) {
      let cell = &mut map[neighbour];
      *cell += 1;
      if *cell == FLASH_THRESHOLD {
        flashes.push(*neighbour);
      }
    }
  }

  for coord in &flashes {
    map[coord] = 0;
  }

  flashes.len()
}

fn neighbours<'a>(coord: &Coord, dest: &'a mut ([Coord; 8])) -> &'a [Coord] {
  match coord {
    Coord { row: 0, col: 0 } => set_neighbours([(0, 1), (1, 0), (1, 1)], dest),
    Coord { row: 0, col: 9 } => set_neighbours([(0, 8), (1, 8), (1, 9)], dest),
    Coord { row: 9, col: 0 } => set_neighbours([(8, 0), (8, 1), (9, 1)], dest),
    Coord { row: 9, col: 9 } => set_neighbours([(8, 8), (8, 9), (9, 8)], dest),
    Coord { row: 0, col } => set_neighbours(
      [
        (0, col - 1),
        (0, col + 1),
        (1, col - 1),
        (1, *col),
        (1, col + 1),
      ],
      dest,
    ),
    Coord { row: 9, col } => set_neighbours(
      [
        (8, col - 1),
        (8, *col),
        (8, col + 1),
        (9, col - 1),
        (9, col + 1),
      ],
      dest,
    ),
    Coord { row, col: 0 } => set_neighbours(
      [
        (row - 1, 0),
        (row - 1, 1),
        (*row, 1),
        (row + 1, 0),
        (row + 1, 1),
      ],
      dest,
    ),
    Coord { row, col: 9 } => set_neighbours(
      [
        (row - 1, 8),
        (row - 1, 9),
        (*row, 8),
        (row + 1, 8),
        (row + 1, 9),
      ],
      dest,
    ),
    Coord { row, col } => set_neighbours(
      [
        (row - 1, col - 1),
        (row - 1, *col),
        (row - 1, col + 1),
        (*row, col - 1),
        (*row, col + 1),
        (row + 1, col - 1),
        (row + 1, *col),
        (row + 1, col + 1),
      ],
      dest,
    ),
  }
}

fn set_neighbours<const N: usize>(
  neighbours: [(usize, usize); N],
  dest: &'_ mut [Coord; 8],
) -> &[Coord] {
  for (to, from) in dest.iter_mut().zip(neighbours) {
    *to = from.into();
  }
  &dest[0..N]
}

impl Map {
  const WIDTH: usize = 10;

  pub fn from_iter<I>(bytes: I) -> io::Result<Self>
  where
    I: Iterator<Item = io::Result<u8>>,
  {
    let mut cells = [0; 100];

    let mut digits = bytes.filter_map(|r| match r {
      err @ Err(_) => Some(err),
      Ok(b @ b'0'..=b'9') => Some(Ok(b - b'0')),
      _ => None,
    });

    for x in &mut cells {
      *x = digits
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, ""))??;
    }

    Ok(Self { cells })
  }

  pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Coord, &mut u8)> {
    self
      .cells
      .iter_mut()
      .enumerate()
      .map(|(idx, value)| (Coord::from(idx), value))
  }
}

impl ops::Index<&Coord> for Map {
  type Output = u8;
  fn index(&self, coord: &Coord) -> &u8 {
    &self.cells[usize::from(coord)]
  }
}

impl ops::IndexMut<&Coord> for Map {
  fn index_mut(&mut self, coord: &Coord) -> &mut u8 {
    &mut self.cells[usize::from(coord)]
  }
}

impl From<usize> for Coord {
  fn from(idx: usize) -> Self {
    Self {
      row: idx / 10,
      col: idx % 10,
    }
  }
}

impl From<(usize, usize)> for Coord {
  fn from((row, col): (usize, usize)) -> Self {
    Self { row, col }
  }
}

impl From<&Coord> for usize {
  fn from(coord: &Coord) -> Self {
    coord.row * 10 + coord.col % 10
  }
}

impl fmt::Debug for Map {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    for chunk in self.cells.chunks(Self::WIDTH) {
      for &c in chunk {
        formatter.write_char(if c < FLASH_THRESHOLD {
          (b'0' + c) as char
        } else {
          '_'
        })?;
      }
      formatter.write_char('\n')?;
    }
    formatter.write_str("----------\n")?;
    Ok(())
  }
}
