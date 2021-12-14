use std::io;
use std::io::BufRead;

pub fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let mut crabs = read_crabs(stdin.lock())?;
  let mid = crabs.len() / 2;
  let (_, nth, _) = crabs.select_nth_unstable(mid);
  let n = *nth;
  let fuel: i32 = crabs.iter().map(|x| (x - n).abs()).sum();

  println!("nth {}, fuel: {}", n, fuel);

  Ok(())
}

fn read_crabs(mut stdin: io::StdinLock) -> io::Result<Vec<i32>> {
  let mut line = String::new();
  stdin.read_line(&mut line)?;
  line.trim().split(',').map(parse_num).collect()
}

fn parse_num(x: &str) -> io::Result<i32> {
  x.parse().map_err(|_| bad_input(x))
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}
