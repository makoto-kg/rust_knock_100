use std::collections::HashMap;
use std::path::Path;
use super::knock030::read_mecab;

pub fn get_verb_surfaces(data: &Vec<Vec<HashMap<String, String>>>) -> Vec<String> {
    data.iter()
        .flat_map(|sentence| sentence.into_iter()
                                     .filter(|elements| elements["pos"] == "動詞")
                                     .map(|elements| elements["surface"].clone()))
        .collect()
}

pub fn exec() {
    let path = Path::new("data/neko.txt.mecab");
    let data = read_mecab(path).unwrap();
    let result = get_verb_surfaces(&data);
    println!("{:?}", result);
}