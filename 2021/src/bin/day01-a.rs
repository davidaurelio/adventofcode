use std::io;
use std::mem;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.read_line(&mut line)?;
    let mut last = line.trim().parse::<u32>().unwrap();
    let mut increases: u32 = 0;
    line.clear();

    while stdin.read_line(&mut line)? > 0 {
        let n = line.trim().parse::<u32>().unwrap();
        increases += (n > mem::replace(&mut last, n)) as u32;
        line.clear();
    }

    println!("Increases: {:?}", increases);
    Ok(())
}
