use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    input = input.trim().to_string();

    if input.contains("0000000") || input.contains("1111111") {
        println!("YES");
    } else {
        println!("NO");
    }
}
