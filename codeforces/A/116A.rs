use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut result = 0;
    let mut aux = 0;

    for _ in 0..n {
        let before: i32 = it.next().unwrap().parse().unwrap();
        let after: i32 = it.next().unwrap().parse().unwrap();

        aux -= before;

        if aux + after > result {
            result = aux + after;
        }

        aux += after;
    }

    println!("{}", result);
}
