use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let first_lower = it.next().unwrap().to_lowercase();
    let first = first_lower.as_bytes();
    let second_lower = it.next().unwrap().to_lowercase();
    let second = second_lower.as_bytes();

    for pos in 0..first.len() {
        if first[pos] > second[pos] {
            println!("1");
            break;
        }

        if first[pos] < second[pos] {
            println!("-1");
            break;
        }
    }

    if first == second {
        println!("0");
    }
}
