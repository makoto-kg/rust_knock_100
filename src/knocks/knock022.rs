use std::path::Path;
use regex::Regex;
use super::knock020::*;

pub fn category_name(article: &Article) -> Vec<&str> {
  let regex = Regex::new(r"\[\[Category:([^|\n]*)\|?.*\]\]").unwrap();
  regex.captures_iter(&article.text).map(|captures| captures.get(1).unwrap().as_str()).collect()
}

pub fn exec() {
  let path = Path::new("data/jawiki-country.json.gz");
  let article = json_read_about(path, "イギリス").unwrap();
  let result = category_name(&article);
  println!("{:?}", result);
}
