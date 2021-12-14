use std::io;

use adventofcode_2021::day06::{count_fish, read_fish};

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  println!(
    "Produced {} fish in 256 days",
    count_fish(read_fish(stdin.lock())?, 256)
  );

  Ok(())
}
