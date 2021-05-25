use std::path::Path;
use std::io::{Result, BufRead, BufReader};
use std::fs::File;

pub fn tab_to_space(path: &Path, tab_width: usize) -> Result<String> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let spaces = " ".repeat(tab_width);
  Ok(br.lines().map(|s| match s { Ok(s) => s.replace("\t", &spaces) + "\n", Err(_) => "\0".to_string() }).collect())
}

pub fn exec() {
  let path = Path::new("data/sample002.txt");
  let result = tab_to_space(path, 4).unwrap();
  println!("{}", result);
}
