use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let plus: i32 = it.next().unwrap().parse().unwrap();
    let mut result = 0;
    let mut current = 1;

    for _ in 0..n {
        let init: i32 = it.next().unwrap().parse().unwrap();
        let finish: i32 = it.next().unwrap().parse().unwrap();

        while current + plus <= init {
            current += plus;
        }

        if current + plus >= init {
            result += finish - current + 1;
            current += finish - current + 1;
        }
    }

    println!("{}", result);
}
