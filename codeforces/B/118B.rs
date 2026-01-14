use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut str_nums = input.split_whitespace();

    let len: usize = str_nums.next().unwrap().parse().unwrap();

    for i in 0..(len + len + 1) {
        if i > len {
            print!("{}", " ".repeat((i - len) * 2));
        } else {
            print!("{}", " ".repeat((len - i) * 2));
        }

        let mut result = String::from("");

        let mut cap = i * 2 + 1;

        if i > len {
            cap = 4 * len + 1 - 2 * i;
        }

        for j in 0..cap {
            if j > cap / 2 {
                result.push_str(&(cap - j - 1).to_string());
            } else {
                result.push_str(&j.to_string());
            }

            if j + 1 != cap {
                result.push(' ');
            }
        }

        println!("{}", result);
    }
}
