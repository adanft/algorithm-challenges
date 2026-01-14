use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.lines();
    let n: i32 = it.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let str = it.next().unwrap();
        let length: usize = str.len();

        if length > 10 {
            let mut chars = str.chars();
            println!("{}{}{}", chars.nth(0).unwrap(), length - 2, chars.nth(length - 2).unwrap());
        } else {
            println!("{}", str);
        }
    }
}
