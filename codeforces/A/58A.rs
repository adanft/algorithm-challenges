use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let word = input.chars();
    let mut counter = 0;

    for char in word {
        match char {
            'h' if counter == 0 => counter += 1,
            'e' if counter == 1 => counter += 1,
            'l' if counter == 2 => counter += 1,
            'l' if counter == 3 => counter += 1,
            'o' if counter == 4 => {
                println!("YES");
                return;
            }
            _ => continue,
        }
    }

    println!("NO");
}
