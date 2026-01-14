use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let p: i32 = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    let mut result = String::new();

    if p - k > 1 {
        result += "<<"
    }

    for ind in p - k..p + k + 1 {
        if ind > n {
            break;
        }

        if ind < 1 {
            continue;
        }

        if result != "" {
            result += " ";
        }

        if ind == p {
            result += &format!("({})", ind);
        } else {
            result += &ind.to_string();
        }
    }

    if p + k < n {
        result += " >>"
    }

    println!("{}", result);
}
