pub fn pi_from_text() -> Vec<usize> {
    let text = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    text.replace(",", "").replace(".", "").split_whitespace().map(|s| s.len()).collect()
}

pub fn exec() {
    let result = pi_from_text();
    println!("{:?}", result);
}