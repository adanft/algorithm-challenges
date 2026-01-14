use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut result = 0;
    let mut it = input.lines();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let mut chars = it.next().unwrap().chars();
    let mut ch = chars.next();

    for _ in 1..n {
        let aux = chars.next();

        if ch == aux {
            result += 1;
        }

        ch = aux;
    }

    println!("{}", result);
}
