use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let chars = input.chars();
    let mut result = String::new();

    for char in chars {
        if char == '7' || char == '4' {
            result += &char.to_string();
        }
    }

    if result.len() == 4 || result.len() == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
