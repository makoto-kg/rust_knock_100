use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

use super::knock020::Article;
use super::knock025::*;

pub fn remove_strong(template: &HashMap<String, String>) -> HashMap<String, String> {
  let regex = Regex::new(r"'{2,5}").unwrap();
  template.iter().map(|(k, v)| {
    (k.clone(), regex.replace_all(v, "").into())
  }).collect()
}

pub fn exec() {
  let path = Path::new("data/sample006.json");
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);

  let article: Article = serde_json::from_reader(reader).unwrap(); 

  let template_obj = template(&article, "基礎情報");

  let result = remove_strong(&template_obj);
  println!("{:?}", result);
}