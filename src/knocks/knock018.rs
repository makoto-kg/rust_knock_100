use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Result};

pub fn sort_by_column(path: &Path, n: usize) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let lines = br.lines().collect::<Result<Vec<_>>>();
    lines.and_then(|mut lines| {
        lines.sort_by(|a, b| a.split_whitespace().nth(n).cmp(&b.split_whitespace().nth(n)));
        Ok(lines)
    })
}

pub fn exec() {
    let path = Path::new("data/sample005.txt");
    let result = sort_by_column(path, 3).unwrap();
    println!("{:?}", result);
}