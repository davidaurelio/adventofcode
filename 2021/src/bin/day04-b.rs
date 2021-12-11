use adventofcode_2021::day04::*;
use std::io;

fn main() -> io::Result<()> {
  let stdin = io::stdin();
  run(stdin.lock(), BingoResultType::Loser)
}
