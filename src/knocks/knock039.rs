use std::path::Path;
use gnuplot::{Figure, AxesCommon};
use super::knock030::read_mecab;
use super::knock036::appearance;

pub fn show_zipf(ranking: &Vec<(usize, String)>) {
    let mut figure = Figure::new();
    figure.axes2d()
        .set_x_log(Some(10.0))
        .set_y_log(Some(10.0))
        .lines(
            1..=ranking.len(),
            ranking.iter()
                .map(|(v, _)| *v)
                .collect::<Vec<_>>(),
            &[]);
    figure.show();
}

pub fn exec() {
    let path = Path::new("data/neko.txt.mecab");
    let data = read_mecab(path).unwrap();
    let apps = appearance(&data);
    let mut pairs: Vec<(usize, String)> = apps.into_iter().map(|(k, v)| (v, k)).collect();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));
    let (pairs_10, _) = pairs.split_at(10);
    println!("{:?}", pairs_10);

    let mut vec = Vec::new();
    vec.extend_from_slice(pairs_10);
    show_zipf(&vec);
}