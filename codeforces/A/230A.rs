use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.trim().lines();

    let mut kirito = it.next().unwrap().split_whitespace();
    let mut strength: i32 = kirito.next().unwrap().parse().unwrap();
    let _ = kirito.next();

    let mut dragons: Vec<(i32, i32)> = it
        .map(|dragon: &str| {
            let mut data = dragon.split_whitespace();
            (
                data.next().unwrap().parse().unwrap(),
                data.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    dragons.sort();

    let mut output = "YES";

    for &(power, bonus) in &dragons {
        if strength > power {
            strength += bonus;
        } else {
            output = "NO";
            break;
        }
    }

    println!("{}", output);
}
