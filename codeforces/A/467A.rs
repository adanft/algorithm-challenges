use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut there_is = 0;

    for _ in 0..n {
        let current: i32 = it.next().unwrap().parse().unwrap();
        let total: i32 = it.next().unwrap().parse().unwrap();

        if current + 2 <= total {
            there_is += 1;
        }
    }

    println!("{}", there_is);
}
