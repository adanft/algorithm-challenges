use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.lines();
    let mut input_condition = it.next().unwrap().split_whitespace();
    let input_points: Vec<&str> = it.next().unwrap().split_whitespace().collect();

    let n: i32 = input_condition.next().unwrap().parse().unwrap();
    let k: usize = input_condition.next().unwrap().parse().unwrap();
    let nk: i32 = input_points[k - 1].parse().unwrap();
    let mut result = 0;

    for pos in 0..n {
        let point: i32 = input_points[pos as usize].parse().unwrap();

        if point > 0 && point >= nk {
            result += 1;
        }
    }

    println!("{}", result);
}
