use std::collections::HashMap;
use std::path::Path;
use gnuplot::Figure;
use super::knock030::read_mecab;
use super::knock036::appearance;

pub fn show_histgram(appearance: &HashMap<String, usize>) {
    let mut histgram = HashMap::new();
    appearance.iter().for_each(|(_, v)| *histgram.entry(v).or_insert(0) += 1);
    let mut histgram = histgram.into_iter().collect::<Vec<_>>();
    histgram.sort_by(|a, b| a.0.cmp(b.0));
    let mut figure = Figure::new();
    figure.axes2d()
            .boxes(
                histgram.iter()
                    .map(|(k, _)| **k)
                    .take(100)
                    .collect::<Vec<_>>(),
                histgram.iter()
                    .map(|(_, v)| *v)
                    .take(100)
                    .collect::<Vec<_>>(),
                &[]);
    figure.show();
}

pub fn exec() {
  let path = Path::new("data/neko.txt.mecab");
  let data = read_mecab(path).unwrap();
  let result = appearance(&data);
  println!("{:?}", result);
}