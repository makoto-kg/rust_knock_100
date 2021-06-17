use super::knock041::Chunk;

pub fn show_src_and_dst(cabocha: &Vec<Vec<Chunk>>) {
    cabocha.iter().for_each(|sentence|
        sentence.iter().for_each(|chunk| if chunk.dst != -1 {
            let chunk_text = chunk.get_text();
            if !chunk_text.is_empty() { println!("{}\t{}", chunk_text, sentence[chunk.dst as usize].get_text()); }
        })
    );
}