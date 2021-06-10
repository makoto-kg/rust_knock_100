use std::collections::HashMap;
use std::path::Path;
use super::knock030::read_mecab;

pub fn a_no_b(data: &Vec<Vec<HashMap<String, String>>>) -> Vec<String> {
    data.iter().filter(|sentence| sentence.len() > 2).flat_map(|sentence| {
        let mut results = Vec::new();
        for i in 0..(sentence.len() -2) {
            if sentence[i + 1]["surface"] == "の"
                && sentence[i]["pos"] == "名詞"
                && sentence[i + 2]["pos"] == "名詞" {
                results.push(sentence[i]["surface"].clone()
                    + "の" + &sentence[i + 2]["surface"])
            }
        }
        results
    }).collect()
}

pub fn exec() {
    let path = Path::new("data/neko.txt.mecab");
    let data = read_mecab(path).unwrap();
    let result = a_no_b(&data);
    println!("{:?}", result);
}