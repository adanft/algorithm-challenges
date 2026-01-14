use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut numbers: Vec<&str> = input.trim().split('+').collect();

    numbers.sort();

    let result = numbers.join("+");

    println!("{}", result);
}
