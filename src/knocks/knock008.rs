pub fn encrypt(text: &str) -> String {
    text.chars().map(|c| if c.is_ascii_lowercase() { (219 - c as u8) as char } else { c }).collect()
}

pub fn exec() {
    let result = encrypt("Makotox500");
    println!("{}", result);
}