use partition::partition;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let mut stdin_lock = stdin.lock();

  let mut line = String::new();
  stdin_lock.read_line(&mut line)?;

  let n = line
    .find(char::is_whitespace)
    .ok_or_else(|| bad_input(&line))?;
  let mut current_mask: u32 = 1 << (n - 1);

  let mut numbers = vec![parse_line(&line)?];
  let mut bit_count = value_for_mask(numbers[0], current_mask);

  while {
    line.clear();
    stdin_lock.read_line(&mut line)? > 0
  } {
    let num = parse_line(&line)?;
    bit_count += value_for_mask(num, current_mask);
    numbers.push(num);
  }

  let is_oxy =
    |x: &u32, mask: u32, bit_count: i32| ((x & mask) != 0) == (bit_count >= 0);
  let (mut oxy, mut co2) =
    partition(&mut numbers, |x| is_oxy(x, current_mask, bit_count));

  while {
    current_mask >>= 1;
    current_mask > 0
  } {
    if oxy.len() > 1 {
      let oxy_bit_count = bit_count_for_mask(oxy.iter(), current_mask);
      oxy = partition(oxy, |x| is_oxy(x, current_mask, oxy_bit_count)).0;
    }

    if co2.len() > 1 {
      let co2_bit_count = bit_count_for_mask(co2.iter(), current_mask);
      co2 = partition(co2, |x| is_oxy(x, current_mask, co2_bit_count)).1;
    }
  }

  println!("oxygen generator rating {:?}", oxy);
  println!("co2 scrubber rating {:?}", co2);

  match oxy.get(0) {
    None => Err(bad_input("no oxy result")),
    Some(o) => match co2.get(0) {
      None => Err(bad_input("no co2 result")),
      Some(c) => {
        println!("Result: {:?}", o * c);
        Ok(())
      }
    },
  }
}

fn value_for_mask(n: u32, bit_mask: u32) -> i32 {
  [-1, 1][(n & bit_mask != 0) as usize]
}

fn bit_count_for_mask(xs: std::slice::Iter<u32>, bit_mask: u32) -> i32 {
  xs.fold(0, |v, x| v + value_for_mask(*x, bit_mask))
}

fn parse_line(line: &str) -> io::Result<u32> {
  u32::from_str_radix(line.trim_end(), 2).map_err(|_| bad_input(line))
}

fn bad_input(line: &str) -> io::Error {
  io::Error::new(io::ErrorKind::InvalidInput, line)
}
