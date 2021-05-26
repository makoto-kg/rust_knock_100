use std::path::Path;
use std::io::{Result, BufRead, BufReader};
use std::fs::File;

pub fn merge_columns(source1: &Path, source2: &Path) -> Result<String> {
  let source1 = File::open(source1)?;
  let source2 = File::open(source2)?;
  let br1 = BufReader::new(source1);
  let br2 = BufReader::new(source2);
  Ok(br1.lines().zip(br2.lines()).map(|(col1, col2)| col1.unwrap() + "\t" + &col2.unwrap() + "\n").collect())
}

pub fn exec() {
  let path1 = Path::new("work/col1.txt");
  let path2 = Path::new("work/col2.txt");

  let result = merge_columns(path1, path2).unwrap();
  println!("{}", result);
}
