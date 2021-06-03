use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use super::knock020::Article;

use regex::Regex;

pub fn files(article: &Article) -> Vec<&str> {
  let regex = Regex::new(r"\[\[(?:File|ファイル):([^|]*).*\]\]").unwrap();
  regex.captures_iter(&article.text).map(|captures| captures.get(1).unwrap().as_str()).collect()
}

pub fn exec() {
  let path = Path::new("data/sample006.json");
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);

  let article: Article = serde_json::from_reader(reader).unwrap();
  let result = files(&article);
  println!("{:?}", result);
}
