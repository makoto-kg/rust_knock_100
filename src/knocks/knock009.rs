use rand::seq::SliceRandom;

pub fn typoglycemia(text: &str) -> String {
    text.split_whitespace().map(|s| {
        if s.len() < 5 {
            s.to_string()
        } else {
            let (head, remaining) = s.split_at(1);
            let (body, tail) = remaining.split_at(remaining.len() - 1);
            let mut body: Vec<_> = body.chars().collect();
            let mut rng = rand::thread_rng();
            body.shuffle(&mut rng);
            head.to_string() + &body.into_iter().collect::<String>() + tail
        }
    }).fold(String::new(), |result, s| if result == "" { s } else { result + " " + &s })
}

pub fn exec() {
    let result = typoglycemia("Thank you for watching");
    println!("{}", result);
}