use std::io::BufRead;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader};

pub fn count_lines(path: &Path) -> std::io::Result<usize> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let mut counter = 0;
  br.lines().for_each(|_| counter += 1);
  Ok(counter)
}

pub fn exec() {
  let path = Path::new("data/sample001.txt");
  let result = count_lines(path).unwrap();
  println!("{}", result);
}