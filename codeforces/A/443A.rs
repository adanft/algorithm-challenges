use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    input = input[1..input.len() - 1].to_string();

    let vector: Vec<&str> = input.trim().split(", ").collect();
    let mut data = String::new();

    for word in vector {
        if !data.contains(word) {
            data += word;
        }
    }

    println!("{}", data.len());
}
