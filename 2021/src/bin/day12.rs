#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
  let cave_system = parse_input(io::stdin().lock())?;

  println!("# paths: {}", find_paths(&cave_system, 0));
  println!("# paths (1 repeat visit): {}", find_paths(&cave_system, 1));

  Ok(())
}

fn find_paths(
  cave_system: &CaveSystem,
  num_repeat_lowercase_visits: u32,
) -> usize {
  let mut finished_paths = Vec::<CaveSystemPath>::new();
  let mut open_paths = vec![CaveSystemPath::start(num_repeat_lowercase_visits)];

  while !open_paths.is_empty() {
    open_paths = open_paths
      .into_iter()
      .flat_map(|p| p.step(cave_system).collect::<Vec<CaveSystemPath>>())
      .filter_map(|p| extract_finished(p, &mut finished_paths))
      .collect();
  }

  finished_paths.len()
}

fn extract_finished(
  path: CaveSystemPath,
  finished_paths: &mut Vec<CaveSystemPath>,
) -> Option<CaveSystemPath> {
  if path.is_complete() {
    finished_paths.push(path);
    None
  } else {
    Some(path)
  }
}

fn parse_input(stdin: io::StdinLock) -> io::Result<CaveSystem> {
  let mut cave_system = CaveSystem::new();

  for line in stdin.lines() {
    cave_system.add_connection(CaveConnection::try_from(line?.as_str())?)
  }

  Ok(cave_system)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Cave {
  name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CaveConnection(Cave, Cave);

#[derive(Clone, Debug, Default)]
struct CaveSystem {
  connections: HashMap<Cave, HashSet<Cave>>,
}

#[derive(Clone, Debug)]
struct CaveSystemPath {
  num_repeat_lowercase_visits: u32,
  visited: HashSet<Cave>,
  last: Cave,
}

impl Cave {
  pub fn end() -> Self {
    Self {
      name: "end".to_string(),
    }
  }
  pub fn start() -> Self {
    Self {
      name: "start".to_string(),
    }
  }

  pub fn is_lowercase(&self) -> bool {
    self
      .name
      .chars()
      .next()
      .map(char::is_lowercase)
      .unwrap_or(false)
  }
}

impl CaveSystem {
  pub fn new() -> Self {
    Self::default()
  }

  fn get_or_insert(&mut self, cave: Cave) -> &mut HashSet<Cave> {
    self.connections.entry(cave).or_insert_with(HashSet::new)
  }

  pub fn add_connection(&mut self, connection: CaveConnection) {
    let a = self.get_or_insert(connection.0.clone());
    if !a.contains(&connection.1) {
      a.insert(connection.1.clone());
    }

    self.get_or_insert(connection.1).insert(connection.0);
  }

  pub fn connections_from(&self, cave: &Cave) -> impl Iterator<Item = &Cave> {
    self.connections[cave].iter()
  }
}

impl CaveSystemPath {
  pub fn start(num_repeat_lowercase_visits: u32) -> Self {
    Self {
      num_repeat_lowercase_visits,
      last: Cave::start(),
      visited: HashSet::from([Cave::start()]),
    }
  }

  pub fn is_complete(&self) -> bool {
    lazy_static! {
      static ref END: Cave = Cave::end();
    }
    self.last == *END
  }

  pub fn step<'a>(
    &'a self,
    cave_system: &'a CaveSystem,
  ) -> impl Iterator<Item = CaveSystemPath> + 'a {
    cave_system.connections_from(&self.last).filter_map(|cave| {
      let mut num_repeat_lowercase_visits = self.num_repeat_lowercase_visits;

      if !self.visited.contains(cave)
        || !cave.is_lowercase()
        || consume_lowercase_visit(cave, &mut num_repeat_lowercase_visits)
      {
        let mut visited = self.visited.clone();
        visited.insert(cave.clone());
        Some(Self {
          num_repeat_lowercase_visits,
          visited,
          last: cave.clone(),
        })
      } else {
        None
      }
    })
  }
}

impl TryFrom<&str> for CaveConnection {
  type Error = io::Error;
  fn try_from(line: &str) -> io::Result<Self> {
    let (from, to) = line
      .split_once('-')
      .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, line))?;

    Ok(CaveConnection(
      Cave {
        name: from.to_string(),
      },
      Cave {
        name: to.to_string(),
      },
    ))
  }
}

fn consume_lowercase_visit(
  cave: &Cave,
  num_repeat_lowercase_visits: &mut u32,
) -> bool {
  if cave != &Cave::start() && *num_repeat_lowercase_visits > 0 {
    *num_repeat_lowercase_visits -= 1;
    true
  } else {
    false
  }
}
