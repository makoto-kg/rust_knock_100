pub fn character_n_gram(text: &str, n: usize) -> Vec<String> {
    let max = text.len() - n + 1;
    let mut result = Vec::new();
    for i in 0..max {
        result.push(text.get(i..(i+n)).unwrap().to_string());
    }

    result
}

pub fn exec() {
    let result = character_n_gram("I have a pen.", 2);
    println!("{:?}", result);
}