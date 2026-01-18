use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.lines();

    let _: usize = it.next().unwrap().parse().unwrap();
    let mut shop_prices: Vec<usize> = it
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let days: usize = it.next().unwrap().parse().unwrap();

    shop_prices.sort();

    for _ in 0..days {
        let price: usize = it.next().unwrap().parse().unwrap();

        println!("{}", shop_prices.partition_point(|x| x <= &price));
    }
}
