use std::io;

fn main() -> io::Result<()> {
  let mut position = [0, 0];

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
      position[0] += amount;
    } else if command == "up" {
      position[1] -= amount;
    } else if command == "down" {
      position[1] += amount;
    } else {
      return Err(bad_input(&line));
    }

    line.clear();
  }

  println!("Result: {:?}", position[0] * position[1]);
  Ok(())
}

fn bad_input(line: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, line)
}
