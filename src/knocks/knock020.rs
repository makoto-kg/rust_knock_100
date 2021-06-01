use std::path::Path;
use std::fs::File;
use serde::Deserialize;
use serde_json::Deserializer;
use flate2::read::GzDecoder;

#[derive(Deserialize, Debug)]
pub struct Article {
  pub text: String,
  pub title: String
}

pub fn json_read_about(path: &Path, about: &str) -> Option<Article> {
  File::open(path).ok()
    .and_then(|file| Some(GzDecoder::new(file)))
    .and_then(|gz| Deserializer::from_reader(gz).into_iter::<Article>()
      .filter(|v| if let Ok(v) = v {v.title == about} else {false})
      .map(|v| v.unwrap())
      .next())
}

pub fn exec() {
  let path = Path::new("data/jawiki-country.json.gz");
  let article = json_read_about(path, "イギリス").unwrap();
  println!("{:?}", article);
}