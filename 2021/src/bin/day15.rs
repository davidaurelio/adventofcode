use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::io::Read;
use std::mem;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Field {
  pub row: usize,
  pub col: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Size {
  pub width: usize,
  pub height: usize,
}

#[derive(Clone, Debug)]
struct RiskMap {
  width: usize,
  fields: Box<[u8]>,
}

#[derive(Clone, Debug)]
struct BitMap {
  size: Size,
  bits: Box<[usize]>,
}

struct TiledMap<'a> {
  repetitions: usize,
  map: &'a RiskMap,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FieldValue<U: Ord>(U, Field);

fn main() -> io::Result<()> {
  let risk_map = RiskMap::try_from(io::stdin().lock().bytes())?;

  println!("Lowest risk (part 1): {}", traverse(&risk_map));
  println!(
    "Lowest risk (part 2): {}",
    traverse(&TiledMap::new(5, &risk_map))
  );
  Ok(())
}

fn traverse<M: Map<u32>>(risk_map: &M) -> u32 {
  let mut visited = BitMap::with_size(risk_map.size());
  let mut open_paths =
    BinaryHeap::from([Reverse(FieldValue(0, visited.start()))]);
  let end = risk_map.end();

  loop {
    let FieldValue(value, field) = open_paths.pop().unwrap().0;
    if field == end {
      break value;
    }

    for neighbour_field in visited
      .neighbour_fields(&field)
      .into_iter()
      .flatten()
      .filter_map(|f| if f.0 { None } else { Some(f.1) })
    {
      let neighbour_value = value + risk_map.get(&neighbour_field);
      visited.enable(&neighbour_field);
      open_paths.push(Reverse(FieldValue(neighbour_value, neighbour_field)));
    }
  }
}

trait Map<T: Ord> {
  fn get(&self, field: &Field) -> T;
  fn size(&self) -> Size;

  fn start(&self) -> Field {
    Field { row: 0, col: 0 }
  }
  fn end(&self) -> Field {
    let size = self.size();
    Field {
      row: size.height - 1,
      col: size.width - 1,
    }
  }

  fn neighbours(&self, field: &Field) -> [Option<Field>; 4] {
    let Field { row, col } = field;
    let Size { width, height } = self.size();

    let top = Some(Field {
      row: row.wrapping_sub(1),
      col: *col,
    });
    let right = Some(Field {
      row: *row,
      col: col + 1,
    });
    let bottom = Some(Field {
      row: row + 1,
      col: *col,
    });
    let left = Some(Field {
      row: *row,
      col: col.wrapping_sub(1),
    });

    match (row, col) {
      (0, 0) => [None, right, bottom, None],
      (0, col) if *col == width - 1 => [None, None, bottom, left],
      (row, 0) if *row == height - 1 => [top, right, None, None],
      (row, col) if *row == height - 1 && *col == width - 1 => {
        [top, None, None, left]
      }
      (0, _) => [None, right, bottom, left],
      (_, 0) => [top, right, bottom, None],
      (row, _) if *row == height - 1 => [top, right, None, left],
      (_, col) if *col == width - 1 => [top, None, bottom, left],
      (_, _) => [top, right, bottom, left],
    }
  }

  fn field_value(&self, field: Field) -> FieldValue<T> {
    FieldValue(self.get(&field), field)
  }

  fn neighbour_fields(&self, field: &Field) -> [Option<FieldValue<T>>; 4] {
    self
      .neighbours(field)
      .map(|o| o.map(|f| self.field_value(f)))
  }
}

impl Map<u32> for RiskMap {
  fn size(&self) -> Size {
    Size {
      width: self.width,
      height: self.fields.len() / self.width,
    }
  }

  fn get(&self, field: &Field) -> u32 {
    if field.col < self.width {
      self.fields[field.row * self.width + field.col].into()
    } else {
      panic!(
        "column {} out of bounds, width is {}",
        field.col, self.width
      );
    }
  }
}

impl<T> TryFrom<io::Bytes<T>> for RiskMap
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
          Ok(RiskMap {
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

impl BitMap {
  const BITS_PER_ELEMENT: usize = mem::size_of::<usize>();

  pub fn with_size(size: Size) -> Self {
    let n_bits = size.width * size.height;
    let n_usize = n_bits / Self::BITS_PER_ELEMENT
      + (n_bits % Self::BITS_PER_ELEMENT != 0) as usize;

    Self {
      size,
      bits: vec![0; n_usize].into_boxed_slice(),
    }
  }

  pub fn enable(&mut self, field: &Field) {
    let (idx, shift) = self.idx_and_shift(field);
    self.bits[idx] |= 1 << shift;
  }

  fn idx_and_shift(&self, field: &Field) -> (usize, usize) {
    if field.col < self.size.width {
      let n = field.col + field.row * self.size.width;
      (n / Self::BITS_PER_ELEMENT, n % Self::BITS_PER_ELEMENT)
    } else {
      panic!(
        "column {} out of bounds, width is {}",
        field.col, self.size.width
      );
    }
  }
}

impl Map<bool> for BitMap {
  fn get(&self, field: &Field) -> bool {
    let (idx, shift) = self.idx_and_shift(field);
    (self.bits[idx] >> shift) & 1 == 1
  }

  fn size(&self) -> Size {
    self.size.clone()
  }
}

impl<'a> TiledMap<'a> {
  pub fn new(repetitions: usize, map: &'a RiskMap) -> Self {
    Self { repetitions, map }
  }
}

impl Map<u32> for TiledMap<'_> {
  fn get(&self, field: &Field) -> u32 {
    let Size { width, height } = self.map.size();
    let tile_x = field.col / width;
    let col = field.col % width;
    let tile_y = field.row / height;
    let row = field.row % height;
    if tile_x < self.repetitions && tile_y < self.repetitions {
      let base = self.map.get(&Field { col, row });
      ((base + tile_x as u32 + tile_y as u32 - 1) % 9) + 1
    } else {
      panic!("field {:?} out of bounds, size is {:?}", field, self.size());
    }
  }
  fn size(&self) -> Size {
    let mut size = self.map.size();
    size.width *= self.repetitions;
    size.height *= self.repetitions;
    size
  }
}
