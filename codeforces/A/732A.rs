use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let mut it = input.split_whitespace();

    let price: i32 = it.next().unwrap().parse().unwrap();
    let coin: i32 = it.next().unwrap().parse().unwrap();
    let mut result = 0;

    for value in 1..10 {
        let total = price * value;

        let total_coin = (total / 10) * 10 + coin;

        if total_coin == total || total_coin - coin == total {
            result = value;
            break;
        }
    }

    println!("{}", result);
}
