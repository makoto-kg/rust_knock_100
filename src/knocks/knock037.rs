use std::path::Path;
use gnuplot::{Figure, Tick, AutoOption, AxesCommon};
use super::knock030::read_mecab;
use super::knock036::appearance;


pub fn show_up_10(top_10: &Vec<(usize, String)>) {
    let mut figure = Figure::new();
    {
        let axes2d = figure.axes2d();
        axes2d.boxes(
            0..top_10.len(),
            top_10.iter().map(|(v, _)| *v).collect::<Vec<_>>(),
            &[]);
        axes2d.set_x_ticks_custom(
            top_10.iter()
                  .enumerate()
                  .map(|(i, (_, k)) | Tick::Major(i, AutoOption::Fix(k.to_string()))),
            &[], &[]);
    }
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
  show_up_10(&vec);
}
