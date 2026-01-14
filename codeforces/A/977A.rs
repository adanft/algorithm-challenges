use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut n: i32 = it.next().unwrap().parse().unwrap();
    let mut k: i32 = it.next().unwrap().parse().unwrap();

    while n > 0 && k > 0 {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1;
        }
        k -= 1;
    }

    println!("{}", n);
}
