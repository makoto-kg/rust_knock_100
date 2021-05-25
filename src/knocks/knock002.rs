pub fn concat_alternately(left: &str, right: &str) -> String {
    let mut result = String::new();
    left.chars().zip(right.chars()).for_each(|(l, r)| {
        result.push(l);
        result.push(r);
    });
    result
}

pub fn exec() {
    let result = concat_alternately("パトカー", "タクシー");
    println!("{}", result);
}