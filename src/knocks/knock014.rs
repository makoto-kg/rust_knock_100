use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

pub fn heads(path: &Path, n: usize) -> Result<String> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  br.lines().take(n).map(|line| line.and_then(|line| Ok(line + "\n"))).collect()
}

pub fn exec() {
  let path = Path::new("data/sample001.txt");
  let result = heads(path, 2).unwrap();
  println!("{}", result);
}
