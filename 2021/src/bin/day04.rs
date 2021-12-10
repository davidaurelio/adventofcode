use std::fmt;
use std::fmt::Write;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use std::vec::Vec;

const BINGO_BOARD_SIZE: usize = 25;

#[derive(Clone, Debug)]
struct BingoBoard {
  set: u32,
  numbers: BingoNumberMap,
}

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let mut stdin_lock = stdin.lock();
  let mut buffer = String::new();

  let numbers = bingo_numbers(&mut stdin_lock, &mut buffer)?;
  let mut boards = bingo_boards(&mut stdin_lock, &mut buffer)?;

  for n in numbers {
    for board in &mut boards {
      if board.check(n) {
        let sum_unmarked = board.sum_unmarked();
        let num: u32 = n.as_u8().into();
        println!(
          "Won on {}, unmarked {}\nResult {}\n{}",
          n.as_u8(),
          sum_unmarked,
          sum_unmarked * num,
          board
        );
        return Ok(());
      }
    }
  }

  Err(io::Error::new(io::ErrorKind::InvalidData, "no board wins"))
}

fn bingo_numbers(
  stdin: &mut io::StdinLock<'_>,
  buffer: &mut String,
) -> io::Result<Vec<BingoNumber>> {
  stdin.read_line(buffer)?;

  buffer.trim().split(',').map(bingo_number).collect()
}

fn bingo_number(str: &str) -> io::Result<BingoNumber> {
  match u8::from_str(str) {
    Err(_) => Err(bad_input(&format!("Not a number: {}", str))),
    Ok(n) => BingoNumber::try_from(n)
      .map_err(|_| bad_input(&format!("Not a valid bingo number: {}", n))),
  }
}

fn bingo_boards(
  stdin: &mut io::StdinLock,
  buffer: &mut String,
) -> io::Result<Vec<BingoBoard>> {
  let mut board_index = BingoBoardIndex::new();
  let mut boards = Vec::with_capacity(100);
  let mut current_board = append_bingo_board(&mut boards);
  let mut did_wrap = BingoBoardIndexWrap::DidNotWrap;

  while {
    buffer.clear();
    stdin.read_line(buffer)? > 0
  } {
    for token in buffer.trim().split_whitespace() {
      if let BingoBoardIndexWrap::DidWrap = did_wrap {
        current_board = append_bingo_board(&mut boards);
      }
      current_board.set(board_index, bingo_number(token)?);
      did_wrap = board_index.advance();
    }
  }

  if let BingoBoardIndexWrap::DidWrap = did_wrap {
    Ok(boards)
  } else {
    println!("{}", boards.last().unwrap());
    Err(io::Error::new(
      io::ErrorKind::InvalidData,
      format!(
        "Found only {} numbers, need {} to fill a board:",
        board_index, BINGO_BOARD_SIZE
      ),
    ))
  }
}

fn append_bingo_board(boards: &mut Vec<BingoBoard>) -> &mut BingoBoard {
  boards.push(BingoBoard::new());
  boards.last_mut().unwrap()
}

fn bad_input(str: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidData, str)
}

impl BingoBoard {
  pub fn new() -> Self {
    Self {
      set: 0,
      numbers: BingoNumberMap::new(),
    }
  }

  pub fn set(&mut self, i: BingoBoardIndex, n: BingoNumber) {
    self.numbers.set_index_of(n, i)
  }

  pub fn check(&mut self, n: BingoNumber) -> bool {
    if let Some(idx) = self.numbers.index_of(n) {
      let bit = idx.as_u8();
      self.set |= 1 << bit;
      self.check_row(bit) || self.check_col(bit)
    } else {
      false
    }
  }

  pub fn sum_unmarked(&self) -> u32 {
    self.numbers.sum_unmarked(self.set)
  }

  fn check_col(&self, bit: u8) -> bool {
    let mask = 0b100001000010000100001 << (bit % 5);
    mask & self.set == mask
  }

  fn check_row(&self, bit: u8) -> bool {
    let mask = 0b11111 << (bit / 5 * 5);
    mask & self.set == mask
  }
}

#[derive(Clone, Copy, Debug)]
struct BingoNumber {
  num: u8,
}

impl BingoNumber {
  pub const fn max_value() -> u8 {
    99
  }

  pub fn as_u8(&self) -> u8 {
    self.num
  }
  pub fn as_usize(&self) -> usize {
    self.as_u8().into()
  }
}

struct BingoNumberTryFromError();
impl TryFrom<u8> for BingoNumber {
  type Error = BingoNumberTryFromError;
  fn try_from(num: u8) -> Result<Self, BingoNumberTryFromError> {
    if num > Self::max_value() {
      Err(BingoNumberTryFromError {})
    } else {
      Ok(Self { num })
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct BingoBoardIndex {
  idx: u8,
}

impl BingoBoardIndex {
  pub fn new() -> Self {
    Self { idx: 0 }
  }

  pub fn advance(&mut self) -> BingoBoardIndexWrap {
    self.idx += 1;
    if usize::from(self.idx) == BINGO_BOARD_SIZE {
      self.idx = 0;
      BingoBoardIndexWrap::DidWrap
    } else {
      BingoBoardIndexWrap::DidNotWrap
    }
  }

  pub fn as_u8(&self) -> u8 {
    self.idx
  }

  pub fn as_usize(&self) -> usize {
    self.idx.into()
  }
}

impl fmt::Display for BingoBoardIndex {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    formatter.write_fmt(format_args!("{}", self.idx))
  }
}

struct BingoBoardIndexTryFromError();
impl TryFrom<u8> for BingoBoardIndex {
  type Error = BingoBoardIndexTryFromError;
  fn try_from(idx: u8) -> Result<Self, BingoBoardIndexTryFromError> {
    if usize::from(idx) < BINGO_BOARD_SIZE {
      Ok(Self { idx })
    } else {
      Err(BingoBoardIndexTryFromError())
    }
  }
}

enum BingoBoardIndexWrap {
  DidNotWrap,
  DidWrap,
}

#[derive(Clone, Debug)]
struct BingoNumberMap {
  map: [u8; Self::SIZE],
}

impl BingoNumberMap {
  const SIZE: usize = (BingoNumber::max_value() + 1) as usize;
  const UNSET: u8 = u8::max_value();

  pub fn new() -> Self {
    Self {
      map: [Self::UNSET; Self::SIZE],
    }
  }

  pub fn index_of(&self, idx: BingoNumber) -> Option<BingoBoardIndex> {
    BingoBoardIndex::try_from(self.map[idx.as_usize()]).ok()
  }

  pub fn set_index_of(&mut self, idx: BingoNumber, value: BingoBoardIndex) {
    self.map[idx.as_usize()] = value.as_u8();
  }

  pub fn sum_unmarked(&self, set_mask: u32) -> u32 {
    self
      .map
      .iter()
      .zip(0..)
      .filter(|(&x, _)| usize::from(x) < BINGO_BOARD_SIZE)
      .filter(|(&x, _)| (set_mask >> x) & 1 == 0)
      .fold(0, |sum, (_, i)| sum + i)
  }
}

impl fmt::Display for BingoBoard {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    if self.set != 0 {
      for i in 0..5 {
        formatter.write_fmt(format_args!(
          "\n {}  {}  {}  {}  {} ",
          bit(self.set, i * 5),
          bit(self.set, i * 5 + 1),
          bit(self.set, i * 5 + 2),
          bit(self.set, i * 5 + 3),
          bit(self.set, i * 5 + 4),
        ))?;
      }
    }
    self.numbers.fmt(formatter)?;
    formatter.write_str("===========\n")
  }
}

impl fmt::Display for BingoNumberMap {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let mut board = [-1i8; BINGO_BOARD_SIZE];
    for n in 0..self.map.len() {
      if let Ok(idx) = BingoBoardIndex::try_from(self.map[n]) {
        board[idx.as_usize()] = n as i8;
      }
    }

    for i in 0..5 {
      formatter.write_fmt(format_args!(
        "\n{:2} {:2} {:2} {:2} {:2}",
        board[i * 5],
        board[i * 5 + 1],
        board[i * 5 + 2],
        board[i * 5 + 3],
        board[i * 5 + 4]
      ))?;
    }

    formatter.write_char('\n')
  }
}

fn bit(set: u32, shift: usize) -> char {
  match (set >> shift) & 1 {
    1 => 'x',
    _ => ' ',
  }
}
