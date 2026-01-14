use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let mut it = input.split_whitespace();

    let num1: i32 = it.next().unwrap().parse().unwrap();
    let num2: i32 = it.next().unwrap().parse().unwrap();
    let mut div: i32 = it.next().unwrap().parse().unwrap();

    let mut result = 1;

    while div > 0 {
        let aux: i32;

        if result == 1 {
            aux = gcd(num1, div);
            result = 0;
        } else {
            aux = gcd(num2, div);
            result = 1;
        }

        div -= aux;
    }

    println!("{}", result);
}

fn gcd(mut num1: i32, mut num2: i32) -> i32 {
    while num2 != 0 {
        let aux = num2;
        num2 = num1 % num2;
        num1 = aux;
    }

    num1
}
