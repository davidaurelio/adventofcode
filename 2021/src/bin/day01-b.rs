use std::io;

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  let mut line = String::new();
  let mut window = [0, 0, 0];

  for i in 0..3 {
    stdin.read_line(&mut line)?;
    window[i] = line.trim().parse().unwrap();
    line.clear();
  }

  let mut window_sum: u32 = window.iter().sum();
  let mut increases: u32 = 0;
  let mut i = 0;
  line.clear();
  while stdin.read_line(&mut line)? > 0 {
    let last = window_sum;
    window_sum -= window[i];
    window[i] = line.trim().parse().unwrap();
    window_sum += window[i];
    i = (i + 1) % 3;
    increases += (window_sum > last) as u32;

    line.clear();
  }

  println!("Increases: {:?}", increases);
  Ok(())
}
