use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, Result};
use super::knock040::Morph;

pub struct Chunk {
    morphs: Vec<Morph>,
    dst: isize,
    srcs: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            morphs: Vec::new(),
            dst: -1,
            srcs: Vec::new(),
        }
    }
    pub fn from_file(path: &Path) -> Result<Vec<Vec<Self>>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut lines = br.lines();

        let mut results = Vec::new();
        let mut sentence = Vec::new();
        let mut chunk = Self::new();
        let mut pairs = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            if line.starts_with("*") {
                if !chunk.morphs.is_empty() {
                    sentence.push(chunk);
                    chunk = Self::new();
                }

                let line = line.split_whitespace().collect::<Vec<_>>();

                let dst = match line[2].trim_end_matches("D").parse::<isize>() {
                    Ok(d) => d,
                    Err(_) => -1
                };
                chunk.dst = dst;
                if let Ok(src) = line[1].parse::<usize>() {
                    pairs.push((src, dst));
                }
            } else if line == "EOS" {
                if !chunk.morphs.is_empty() {
                    sentence.push(chunk);
                    chunk = Self::new();
                }

                pairs.into_iter()
                    .filter(|(_, dst)| *dst != -1)
                    .for_each(|(src, dst) | sentence[dst as usize].srcs.push(src));
                pairs = Vec::new();

                if !sentence.is_empty() {
                    results.push(sentence);
                    sentence = Vec::new();
                }
            } else {
                // chunk.morphs.push(Morph::from_line(&line));
            }
        }
        
        Ok(results)
    }
}