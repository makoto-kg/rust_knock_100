use std::collections::HashMap;
use std::path::Path;
use super::knock030::read_mecab;

pub fn longest_noun(data: &Vec<Vec<HashMap<String, String>>>) -> String {
    let mut longest = String::new();
    let mut max_length = 0;
    let mut tmp = String::new();
    let mut current_length = 0;

    data.into_iter().for_each(|sentence| {
        let mut buffer = String::new();
        if tmp.is_empty() {
            buffer.push_str(&tmp);
            tmp = String::new();
        }
        sentence.into_iter().for_each(|elements| {
            if elements["pos"] == "名詞" {
                buffer.push_str(&elements["surface"]);
                current_length += 1;
            } else {
                if current_length > max_length {
                    max_length = current_length;
                    longest = buffer.clone();
                }
                current_length = 0;
                buffer = String::new();
            }
        });
        if !buffer.is_empty() {
            tmp = buffer;
        }
    });

    longest
}

pub fn exec() {
    let path = Path::new("data/neko.txt.mecab");
    let data = read_mecab(path).unwrap();
    let result = longest_noun(&data);
    println!("{}", result);
}