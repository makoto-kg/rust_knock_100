use std::fmt::Display;

pub fn generate_template_text<X: Display, Y: Display, Z: Display>(x: X, y: Y, z: Z) -> String {
    format!("{}時の{}は{}", x, y, z)
}

pub fn exec() {
    let result = generate_template_text("3", "おやつ", "どらやき");
    println!("{}", result);
}