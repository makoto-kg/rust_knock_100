pub fn odd_chars(text: &str) -> String {
    text.chars().step_by(2).collect()
}

pub fn exec() {
    let result = odd_chars("パタトクカシーー");
    println!("{}", result);
}