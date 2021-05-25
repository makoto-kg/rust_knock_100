pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

pub fn exec() {
    let result = reverse("stressed");
    println!("{}", result);
}