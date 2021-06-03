use super::knock020::*;


use std::path::Path;
use std::io::BufReader;
use std::fs::File;

use regex::Regex;

pub fn sections(article: &Article) -> Vec<(usize, &str)> {
  let regex = Regex::new(r"^\s*(=+)\s*([^=]+)\s*=+").unwrap();
  article.text.lines()
    .filter(|line| regex.is_match(line))
    .map(|line| regex.captures(line).unwrap())
    .map(|captures| (captures.get(1).unwrap().as_str().len() - 1, captures.get(2).unwrap().as_str()))
    .collect()
}

pub fn exec() {
  let path = Path::new("data/sample006.json");
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);

  let article: Article = serde_json::from_reader(reader).unwrap();
  let result = sections(&article);
  println!("{:?}", result);
}
