use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let chars = input.chars();
    let mut result = String::new();

    for char in chars {
        if !"aeiouyAEIOUY".contains(char) {
            result += &format!(".{}", char.to_lowercase())
        }
    }

    println!("{}", result);
}
