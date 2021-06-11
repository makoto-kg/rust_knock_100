use std::collections::HashMap;
use std::path::Path;
use super::knock030::read_mecab;

pub fn appearance(data: &Vec<Vec<HashMap<String, String>>>) -> HashMap<String, usize> {
  let mut appearance = HashMap::new();

  data.iter()
      .for_each(|sentence| {
        sentence.into_iter().for_each(|elements| {
          *appearance.entry(elements["surface"].clone()).or_insert(0) += 1; 
        })
      });

  appearance
}

pub fn exec() {
  let path = Path::new("data/neko.txt.mecab");
  let data = read_mecab(path).unwrap();
  let result = appearance(&data);
  println!("{:?}", result);
}