use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut str_nums = input.split_whitespace();

    let code = str_nums.next().unwrap().chars();
    let mut result = 0;

    for char in code {
        match char {
            'H' => result += 1,
            'Q' => result += 1,
            '9' => result += 1,
            _ => (),
        }
    }

    if result == 0 {
        println!("NO");
    } else {
        println!("YES");
    }
}
