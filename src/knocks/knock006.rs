use std::collections::HashSet;
use super::knock005;

pub fn exec() {
    let paraparaparadise = knock005::character_n_gram("paraparaparadise", 2).into_iter().collect::<HashSet<String>>(); 
    let paragraph = knock005::character_n_gram("paragraph", 2).into_iter().collect::<HashSet<String>>();
    let union = &paraparaparadise | &paragraph;
    println!("和集合={:?}, 積集合={:?}, 差集合={:?}",
             union,
             &paraparaparadise & &paragraph,
             &paraparaparadise - &paragraph
    );
}