use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut limak: i32 = it.next().unwrap().parse().unwrap();
    let mut bob: i32 = it.next().unwrap().parse().unwrap();
    let mut years = 0;

    while limak <= bob {
        years += 1;

        limak *= 3;
        bob *= 2;
    }

    println!("{}", years);
}
