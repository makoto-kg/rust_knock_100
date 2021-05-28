use std::path::Path;
use std::io::{BufReader, Result, BufRead, Error, ErrorKind};
use std::fs::File;
use std::collections::HashSet;

pub fn get_column_difference(path: &Path, n: usize) -> Result<HashSet<String>> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let mut result = HashSet::new();

  br.lines().map(|line|
    line.and_then(|line|
      line.split_whitespace().map(|word| word.to_string()).nth(n)
      .ok_or(Error::new(ErrorKind::NotFound, format!("the column {} is not fund.", n)))))
  .for_each(|line| match line {
    Ok(line) => { result.insert(line.to_string()); },
    Err(e) => eprintln!("{}", e)
  });

  Ok(result)
}

pub fn exec() {
  let path = Path::new("data/sample004.txt");
  let result = get_column_difference(path, 2).unwrap();
  println!("{:?}", result);
}