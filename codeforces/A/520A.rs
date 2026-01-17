use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let _ = it.next();
    let word = it.next().unwrap();

    if word.len() < 26 {
        println!("NO");
    } else {
        let letters: HashSet<char> = word.to_lowercase().chars().collect();

        if letters.len() == 26 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
