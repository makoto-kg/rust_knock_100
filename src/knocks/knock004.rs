use std::collections::HashMap;

pub fn element_symbols() -> HashMap<String, usize> {
    let text = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let mut result = HashMap::new();
    text.replace(".", "").split_whitespace().enumerate().for_each(|(i, s)| {
        let symbol = match i {
            0 | 4 | 5 | 6 | 7 | 8 | 14 | 15 | 18 => s.get(0..1).unwrap(),
            _ => s.get(0..2).unwrap()
        };
        result.insert(symbol.to_string(), i + 1);
    });
    result
}

pub fn exec() {
    let result = element_symbols();
    println!("{:?}", result);
}