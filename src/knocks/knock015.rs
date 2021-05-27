use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

pub fn tails(path: &Path, n: usize) -> Result<String> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let lines = br.lines().collect::<Vec<_>>();
  let n_lines = lines.into_iter().rev().take(n).collect::<Vec<_>>();
  n_lines.into_iter().rev().map(|line| line.and_then(|line| Ok(line + "\n"))).collect()
}

pub fn exec() {
  let path = Path::new("data/sample001.txt");
  let result = tails(path, 2).unwrap();
  println!("{}", result);
}