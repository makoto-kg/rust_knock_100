use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, Write, Result, Error, ErrorKind};

pub fn get_col(source: &Path, out: &Path, column_number: usize) -> Result<()> {
  let source = File::open(source)?;
  let out = OpenOptions::new().write(true).create(true).truncate(true).open(out)?;
  let br = BufReader::new(source);
  let mut bw = BufWriter::new(out);
  br.lines().map(|line| {
    if let Ok(line) = line {
      match line.split_whitespace().nth(column_number) {
        Some(word) => Ok(word.to_string() + "\n"),
        None => Err(Error::new(ErrorKind::NotFound, format!("the column {} is not found.", column_number)))
      }
    } else {
      line
    }
  }).for_each(|col| { let _ = col.and_then(|col| bw.write(col.as_bytes())); });
  Ok(())
}

pub fn exec() {
  let input = Path::new("data/sample003.txt");
  let out1 = Path::new("work/col1.txt");
  let out2 = Path::new("work/col2.txt");

  let result = get_col(&input, &out1, 0).unwrap();
  println!("{:?}", result);
  let result = get_col(&input, &out2, 1);
  println!("{:?}", result);
}