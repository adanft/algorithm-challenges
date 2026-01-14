use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut result = 0;
    let mut it = input.lines();
    let n: i32 = it.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let number_str = it.next().unwrap();
        if number_str.matches("1").count() >= 2 {
            result += 1;
        }
    }

    println!("{}", result);
}
