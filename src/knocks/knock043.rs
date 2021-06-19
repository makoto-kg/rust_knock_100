use super::knock041::Chunk;

pub fn noun_verb(cabocha: &Vec<Vec<Chunk>>) {
    cabocha.iter().for_each(|sentence| {
        sentence.iter()
            .filter(|chunk| {
                chunk.dst != -1
                && chunk.morphs.iter().find(|m| m.pos == "名刺").is_some()
                && sentence[chunk.dst as usize].morphs.iter().find(|m| m.pos == "動詞").is_some()
            })
            .for_each(|chunk| println!("{}\t{}", chunk.get_text(), sentence[chunk.dst as usize].get_text()));
    });
}

pub fn exec() {
    println!("{}", "Hi!");
}