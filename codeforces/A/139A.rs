use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut pages: i32 = it.next().unwrap().parse().unwrap();
    let pages_to_read: Vec<i32> = it.map(|x| x.parse().unwrap()).collect();

    let mut counter = 1;
    let mut day = 1;

    while pages > 0 {
        pages -= pages_to_read[counter - 1];
        day = counter;
        counter += 1;

        if counter > pages_to_read.len() {
            counter = 1
        }
    }

    println!("{}", day);
}
