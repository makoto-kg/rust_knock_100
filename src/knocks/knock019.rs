use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufReader, BufRead, Result};

pub fn sort_by_frequency(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines = br.lines().collect::<Result<Vec<_>>>();
    let mut counter: HashMap<String, usize> = HashMap::new();
    lines.and_then(|lines| {
        Ok(lines.into_iter().for_each(|line| {
            let key = line.split_whitespace().next().unwrap_or("");
            if counter.contains_key(key) {
                *counter.get_mut(key).unwrap() += 1
            } else {
                counter.insert(key.to_string(), 1);
            }
        }))
    }).and_then(|_| {
        let mut tmp = counter.into_iter().collect::<Vec<_>>();
        tmp.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(tmp.into_iter().map(|(k, _)| k).collect())
    })
}

pub fn exec() {
    let path = Path::new("data/sample004.txt");
    let result = sort_by_frequency(path).unwrap();
    println!("{:?}", result);
}