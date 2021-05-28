use std::path::Path;
use std::fs::File;
use std::io::{BufReader, Result, BufRead};

pub fn split_file(path: &Path, n: usize) -> Result<Vec<String>> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let lines = br.lines().collect::<Result<Vec<_>>>();
  lines.and_then(|lines| Ok(lines.chunks(n).map(|chunk| chunk.join("\n")).collect()))
}

pub fn exec() {
  let path = Path::new("data/sample001.txt");
  let result = split_file(path, 2).unwrap();
  println!("{:?}", result);
}