use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let mut it = input.split_whitespace();

    let friends: i32 = it.next().unwrap().parse().unwrap();
    let fence_h: i32 = it.next().unwrap().parse().unwrap();
    let mut total = 0;

    for _ in 0..friends {
        let height: i32 = it.next().unwrap().parse().unwrap();

        if height > fence_h {
            total += 2;
        } else {
            total += 1;
        }
    }

    println!("{}", total);
}
