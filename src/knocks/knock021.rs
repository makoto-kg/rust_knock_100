use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use super::knock020::Article;

pub fn category_line(article: &Article) -> Vec<&str> {
  let regex = Regex::new(r"\[\[Category.*\]\]").unwrap(); 
  article.text.lines().filter(|l| regex.is_match(l)).collect()
}

pub fn exec() {
  let path = Path::new("data/sample006.json");
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);

  let article: Article = serde_json::from_reader(reader).unwrap();

  let result = category_line(&article);
  println!("{:?}", result);
}