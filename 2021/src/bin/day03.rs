use std::io;
use std::vec::Vec;

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let mut line = String::new();

  stdin.read_line(&mut line)?;
  let n = line
    .find(char::is_whitespace)
    .ok_or_else(|| bad_input(&line))?;
  let mut counts = [0].repeat(n);
  count(&line, &mut counts)?;
  line.clear();

  while stdin.read_line(&mut line)? > 0 {
    count(&line, &mut counts)?;
    line.clear();
  }

  let mut gamma = 0u32;
  for count in counts {
    gamma = (gamma << 1) | (count >= 0) as u32;
  }

  let epsilon = !gamma & ((1 << n) - 1);

  println!("Result: {:?}", epsilon * gamma);

  Ok(())
}

fn count(line: &str, counts: &mut Vec<i32>) -> io::Result<()> {
  let digits = line.trim_end();

  if digits.len() != counts.len() {
    return Err(bad_input(&line));
  }

  for (digit, count) in digits.chars().zip(counts) {
    match digit {
      '0' => *count -= 1,
      '1' => *count += 1,
      _ => return Err(bad_input(&line)),
    }
  }

  Ok(())
}

fn bad_input(line: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, line)
}
