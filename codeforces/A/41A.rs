use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.lines();
    let word = it.next().unwrap();
    let word_trans = it.next().unwrap();
    let mut new_word = String::with_capacity(word.len());
    let mut chars = word.chars();

    for _ in 0..word.len() {
        new_word.insert(0, chars.next().unwrap());
    }

    if new_word == word_trans {
        println!("YES");
    } else {
        println!("NO");
    }
}
