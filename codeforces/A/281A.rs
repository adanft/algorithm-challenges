use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let word = input.trim();
    let mut chars = word.chars();

    let new_word = format!(
        "{}{}",
        chars.next().unwrap().to_uppercase(),
        word.get(1..).unwrap()
    );

    println!("{}", new_word);
}
