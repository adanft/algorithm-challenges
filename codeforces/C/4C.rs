use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let len: i32 = it.next().unwrap().parse().unwrap();
    let mut words: HashMap<&str, i32> = HashMap::new();

    for _ in 0..len {
        let word: &str = it.next().unwrap();
        words
            .entry(word)
            .and_modify(|value| {
                *value += 1;
                println!("{}{}", word, value);
            })
            .or_insert_with(|| {
                println!("OK");
                0
            });
    }
}
