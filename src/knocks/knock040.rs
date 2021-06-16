use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

#[derive(Debug)]
pub struct Morph {
    pub surface: String,
    pub base: String,
    pub pos: String,
    pub pos1: String
}

impl Morph {
    pub fn from_file(path: &Path) -> Result<Vec<Vec<Self>>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut lines = br.lines().filter(|line| if let Ok(line) = line {!line.starts_with('*') } else { false });
        let mut results = Vec::new();
        let mut buffer = Vec::new(); 

        while let Some(Ok(line)) = lines.next() {
            if line == "EOS" {
                if !buffer.is_empty() { results.push(buffer); }
                buffer = Vec::new();
            } else {
                let line = line.replace("\t", ",");
                let tmp: Vec<_> = line.split_terminator(',').collect();
                buffer.push(Self {
                    surface: tmp[0].to_string(),
                    base: tmp[7].to_string(),
                    pos: tmp[1].to_string(),
                    pos1: tmp[2].to_string(),
                });
            }
        }

        Ok(results)
    }
}

pub fn exec() {
    let path = Path::new("./data/neko.txt.cabocha");
    let result = Morph::from_file(path).unwrap();
    println!("{:?}", result);
}