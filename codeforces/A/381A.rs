use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let _: i32 = it.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = it.map(|x| x.parse().unwrap()).collect();

    let mut sereja: i32 = 0;
    let mut dima: i32 = 0;

    let mut left = 0;
    let mut right = nums.len() - 1;

    for ind in 0..nums.len() {
        let aux;

        if nums[left] >= nums[right] {
            aux = nums[left];
            left += 1;
        } else {
            aux = nums[right];
            right -= 1;
        }

        if ind % 2 == 0 {
            sereja += aux;
        } else {
            dima += aux;
        }
    }

    println!("{} {}", sereja, dima);
}
