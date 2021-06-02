use std::path::Path;
use regex::Regex;
use super::knock020::*;

pub fn category_line(article: &Article) -> Vec<&str> {
  let regex = Regex::new(r"\[\[Category.*\]\]").unwrap(); 
  article.text.lines().filter(|l| regex.is_match(l)).collect()
}

pub fn exec() {
  let path = Path::new("data/jawiki-country.json.gz");
  let article = json_read_about(path, "イギリス").unwrap();
  let result = category_line(&article);
  println!("{:?}", result);
}