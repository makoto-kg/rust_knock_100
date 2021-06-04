use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

use super::knock020::Article;

pub fn template(article: &Article, template_name: &str) -> HashMap<String, String> {
  let wide_regex = Regex::new(&(r"(?sm)^\{\{".to_owned() + template_name + r"\s*(.*)^}}$")).unwrap();
  let narrow_regex = Regex::new(r"(?sm)^\|\s*([^\n]*)\s*=\s*([^\n]*)\n(.*)").unwrap();
  let mut result = HashMap::new();

  wide_regex.captures_iter(&article.text)
    .map(|captures| captures.get(1).unwrap().as_str())
    .for_each(|text| {
      let mut text = text;
      while narrow_regex.is_match(text) {
        narrow_regex.captures_iter(&text).for_each(|captures| {
          result.insert(captures.get(1).unwrap().as_str().to_string(), captures.get(2).unwrap().as_str().to_string());
          text = captures.get(3).unwrap().as_str();
        });
      }
    });

  result
}

pub fn exec() {
  let path = Path::new("data/sample006.json");
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);

  let article: Article = serde_json::from_reader(reader).unwrap(); 

  let result = template(&article, "基礎情報");
  println!("{:?}", result);
}