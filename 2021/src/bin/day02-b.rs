use std::io;

fn main() -> io::Result<()> {
  let mut aim = 0;
  let mut depth = 0;
  let mut position = 0;

  let stdin = io::stdin();
  let mut line = String::new();

  while stdin.read_line(&mut line)? > 0 {
    let mut words = line.split_whitespace();

    let command = match words.next() {
      Some(word) => word,
      None => return Err(bad_input(&line)),
    };

    let amount = match words.next() {
      Some(word) => match word.parse::<u32>() {
        Ok(number) => number,
        Err(_) => return Err(bad_input(&line)),
      },
      None => return Err(bad_input(&line)),
    };

    if command == "forward" {
      position += amount;
      depth += amount * aim;
    } else if command == "up" {
      aim -= amount;
    } else if command == "down" {
      aim += amount;
    } else {
      return Err(bad_input(&line));
    }

    line.clear();
  }

  println!("Result: {:?}", position * depth);
  Ok(())
}

fn bad_input(line: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, line)
}
