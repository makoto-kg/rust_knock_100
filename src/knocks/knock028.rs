use curl::easy::{Easy2, Handler, WriteError};
use serde_json::value::Value;

struct Collector(Vec<u8>);


impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}

pub fn get_image(file_name: &str) -> String {
  let mut curl = Easy2::new(Collector(Vec::new()));
  curl.get(true).unwrap();
  curl.url(&("https://commons.wikimedia.org/w/api.php?action=query&titles=File:".to_string()
           + &percent_encoding::utf8_percent_encode(file_name, percent_encoding::NON_ALPHANUMERIC).to_string()
           + "&prop=imageinfo&iiprop=url&format=json")).unwrap();
  curl.perform().unwrap();
  let collected = curl.get_ref();
  let json: Value = serde_json::from_slice(&collected.0).unwrap();
  json["query"]["pages"].as_object().unwrap().values().next().unwrap()["imageinfo"][0]["url"].to_string()
}

pub fn exec() {
  let result = get_image("japan");
  println!("{}", result);
}