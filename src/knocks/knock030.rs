use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::io::{Result, BufReader, BufRead};

pub fn read_mecab(path: &Path) -> Result<Vec<Vec<HashMap<String, String>>>> {
  let file = File::open(path)?;
  let br = BufReader::new(file);
  let mut lines = br.lines();
  let mut results = Vec::new();
  let mut sentence = Vec::new();

  while let Some(Ok(line)) = lines.next() {
    if line == "EOS" {
      results.push(sentence);
      sentence = Vec::new();
    } else {
      let mut elements = HashMap::new();
      let line = line.replace("\t", ",");
      let tmp: Vec<_> = line.split_terminator(',').collect();
      elements.insert("surface".to_string(), tmp[0].to_string());
      elements.insert("base".to_string(), tmp[7].to_string());
      elements.insert("pos".to_string(), tmp[1].to_string());
      elements.insert("pos1".to_string(), tmp[2].to_string());
      sentence.push(elements);
    }
  }

  Ok(results)
}

pub fn exec() {
  let path = Path::new("data/neko.txt.mecab");
  let result = read_mecab(path);

  println!("{:?}", result);
}