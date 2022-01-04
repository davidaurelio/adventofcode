use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::io::Read;
use std::mem;
use std::ops::Index;
use std::ops::IndexMut;
type Field = (usize, usize);

#[derive(Clone, Debug)]
struct Map<U> {
  width: usize,
  fields: Box<[U]>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FieldValue<U: Ord>(U, Field);

fn main() -> io::Result<()> {
  let risk_map = Map::try_from(io::stdin().lock().bytes())?;
  let mut accum_risk_map = Map::<u32>::with_size(risk_map.size());
  let mut open_paths =
    BinaryHeap::from([Reverse(accum_risk_map.start_value())]);
  let end = dbg!(accum_risk_map.end());
  let value = loop {
    let FieldValue(value, field) = open_paths.pop().unwrap().0;
    println!("Going to {:?} ({})", field, value);
    if field == end {
      break value;
    }

    for neighbour_field in accum_risk_map
      .neighbour_fields(field)
      .into_iter()
      .flatten()
      .filter_map(|f| if f.0 != 0 { None } else { Some(f.1) })
    {
      let neighbour_value = value + risk_map[&neighbour_field] as u32;
      accum_risk_map[&neighbour_field] = neighbour_value;
      open_paths.push(Reverse(FieldValue(neighbour_value, neighbour_field)));
    }
  };

  println!("Lowest risk: {}", value);

  Ok(())
}

impl<U> Map<U>
where
  U: Ord,
{
  pub fn start(&self) -> Field {
    (0, 0)
  }

  pub fn end(&self) -> Field {
    (self.height() - 1, self.width - 1)
  }
}

impl<U> Map<U> {
  pub fn neighbours(&self, (row, col): Field) -> [Option<Field>; 4] {
    let height = self.height();
    let top = Some((row.wrapping_sub(1), col));
    let right = Some((row, col + 1));
    let bottom = Some((row + 1, col));
    let left = Some((row, col.wrapping_sub(1)));

    match (row, col) {
      (0, 0) => [None, right, bottom, None],
      (0, col) if col == self.width - 1 => [None, None, bottom, left],
      (row, 0) if row == height - 1 => [top, right, None, None],
      (row, col) if row == height - 1 && col == self.width - 1 => {
        [top, None, None, left]
      }
      (0, _) => [None, right, bottom, left],
      (_, 0) => [top, right, bottom, None],
      (row, _) if row == height - 1 => [top, right, None, left],
      (_, col) if col == self.width - 1 => [top, None, bottom, left],
      (_, _) => [top, right, bottom, left],
    }
  }

  pub fn height(&self) -> usize {
    self.fields.len() / self.width
  }

  pub fn size(&self) -> (usize, usize) {
    (self.width, self.height())
  }
}

impl<U: Copy + Ord> Map<U> {
  pub fn start_value(&self) -> FieldValue<U> {
    let start = self.start();
    FieldValue(self[&start], start)
  }
  pub fn neighbour_fields(&self, field: Field) -> [Option<FieldValue<U>>; 4] {
    self
      .neighbours(field)
      .map(|o| o.map(|n| FieldValue(self[&n], n)))
  }
}

impl<U> Map<U>
where
  U: Default,
{
  pub fn with_size((width, height): (usize, usize)) -> Self {
    Self {
      width,
      fields: {
        let len = width * height;
        let mut fields = Vec::with_capacity(len);
        fields.resize_with(len, U::default);
        fields.into_boxed_slice()
      },
    }
  }
}

impl<U> Index<&Field> for Map<U> {
  type Output = U;
  fn index(&self, (row, col): &Field) -> &Self::Output {
    if *col > self.width {
      panic!("column {} out of bounds, width is {}", col, self.width);
    }
    &self.fields[row * self.width + col]
  }
}

impl<U> IndexMut<&Field> for Map<U> {
  fn index_mut(&mut self, (row, col): &Field) -> &mut Self::Output {
    if *col > self.width {
      panic!("column {} out of bounds, width is {}", col, self.width);
    }
    &mut self.fields[row * self.width + col]
  }
}

impl<T> TryFrom<io::Bytes<T>> for Map<u8>
where
  T: std::io::Read,
{
  type Error = io::Error;

  fn try_from(bytes: io::Bytes<T>) -> io::Result<Self> {
    let mut width = None;
    let mut num_bytes = 0_usize;
    let mut fields = Vec::new();

    for byte in bytes {
      match byte? {
        b @ b'1'..=b'9' => {
          num_bytes += 1;
          fields.push(b - b'0');
          Ok(())
        }
        b'\n' => match width {
          None => {
            width = Some(mem::replace(&mut num_bytes, 0));
            Ok(())
          }
          Some(w) => {
            if w == mem::replace(&mut num_bytes, 0) {
              Ok(())
            } else {
              Err(invalid_data("Rows width different widths in input"))
            }
          }
        },
        _ => Err(invalid_data("Invalid byte in input")),
      }?
    }

    match width {
      None => Err(invalid_data("No newline in file")),
      Some(0) => Err(invalid_data("Empty map")),
      Some(width) => {
        if num_bytes == 0 || num_bytes == width {
          Ok(Map {
            width,
            fields: fields.into_boxed_slice(),
          })
        } else {
          Err(invalid_data("Last row is too short"))
        }
      }
    }
  }
}

fn invalid_data(desc: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidData, desc)
}
