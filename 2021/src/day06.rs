use std::io;
use std::io::BufRead;

pub fn read_fish(mut stdin: io::StdinLock, days: i32) -> io::Result<Vec<i32>> {
  let mut line = String::new();
  stdin.read_line(&mut line)?;
  line
    .trim()
    .split(',')
    .map(|x| parse_num(x).map(|n| days - n))
    .collect()
}

fn parse_num(x: &str) -> io::Result<i32> {
  x.parse().map_err(|_| bad_input(x))
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}

pub fn count_fish(mut fish: Vec<i32>) -> usize {
  let mut count = 0usize;
  while let Some(mut f) = fish.pop() {
    count += 1;
    while f > 0 {
      fish.push(f - 9);
      f -= 7;
    }
  }
  count
}
