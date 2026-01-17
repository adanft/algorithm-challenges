use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let len: usize = it.next().unwrap().parse().unwrap();
    let mut groups: Vec<Vec<usize>> = vec![vec![]; 3];

    for ind in 0..len {
        let group: usize = it.next().unwrap().parse().unwrap();
        groups[group - 1].push(ind + 1);
    }

    let total = groups.iter().map(|group| group.len()).min().unwrap();

    println!("{}", total);

    for pos in 0..total {
        println!("{} {} {}", groups[0][pos], groups[1][pos], groups[2][pos]);
    }
}
