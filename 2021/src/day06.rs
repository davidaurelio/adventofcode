use std::io;
use std::io::BufRead;

type FishMap = [usize; 9];

pub fn read_fish(mut stdin: io::StdinLock) -> io::Result<FishMap> {
  let mut map = [0; 9];
  let mut line = String::new();
  stdin.read_line(&mut line)?;

  line
    .trim()
    .split(',')
    .map(parse_num)
    .try_for_each(|x| x.map(|n| map[n] += 1))?;
  Ok(map)
}

fn parse_num(x: &str) -> io::Result<usize> {
  x.parse().map_err(|_| bad_input(x))
}

fn bad_input(input: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, input)
}

pub fn count_fish(mut fish: FishMap, mut days: usize) -> usize {
  while days > 0 {
    let zeros = fish[0];
    fish.copy_within(1.., 0);
    fish[8] = zeros;
    fish[6] += zeros;
    days -= 1;
  }

  fish.iter().sum()
}
