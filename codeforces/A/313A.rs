use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    let num: i32 = input.parse().unwrap();

    let option1 = num / 10;
    let option2 = ((num / 100) * 10) + (num % 10);

    if num > 0 {
        println!("{}", num);
    } else if option1 > option2 {
        println!("{}", option1);
    } else {
        println!("{}", option2);
    }
}
