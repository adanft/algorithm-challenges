use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let piles: usize = it.next().unwrap().parse().unwrap();
    let mut worms: Vec<usize> = Vec::with_capacity(piles);
    let mut prev = 0;

    for _ in 0..piles {
        let worm: usize = it.next().unwrap().parse().unwrap();

        worms.push(worm + prev);
        prev += worm;
    }

    let juicy_worms: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..juicy_worms {
        let label: usize = it.next().unwrap().parse().unwrap();
        let pile_n = worms.partition_point(|x| x < &label);

        println!("{}", pile_n + 1);
    }
}
