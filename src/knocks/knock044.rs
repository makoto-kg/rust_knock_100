use std::io::{Result, BufWriter, Write};
use std::path::Path;
use std::fs::{OpenOptions};
use super::knock041::Chunk;

pub fn to_graph(sentence: &Vec<Chunk>, out_file: &Path) -> Result<()> {
    let file = OpenOptions::new().write(true).create(true).open(out_file)?;
    let mut bw = BufWriter::new(file);
    bw.write(b"digraph cabocha{\n")?;
    sentence.iter().map(|chunk| {
        bw.write(chunk.morphs.iter().map(|m| m.surface.clone()).collect::<String>().as_bytes())?;
        if chunk.dst != -1 {
            bw.write(b" -> ")?;
            bw.write(sentence[chunk.dst as usize].morphs.iter().map(|m| m.surface.clone()).collect::<String>().as_bytes())?;
            bw.write(b";\n")?;
        }
        Ok(())
    }).collect::<Result<()>>()?;
    bw.write(b"}")?;
    Ok(())
}

pub fn exec() {
    println!("Hi 044");
}