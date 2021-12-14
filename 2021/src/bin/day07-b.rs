use std::io;
use std::io::BufRead;

pub fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let crabs = read_crabs(stdin.lock())?;
  let n = crabs.len() as i32;
  let avg: i32 = crabs.iter().sum::<i32>() / n;
  let fuel: i32 = crabs.iter().map(|x| cost((x - avg).abs())).sum();

  println!("avg {}, fuel: {}", avg, fuel);

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

fn cost(distance: i32) -> i32 {
  distance * (distance + 1) / 2
}
