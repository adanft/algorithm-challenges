use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut str_nums = input.split_whitespace();

    let n: i32 = str_nums.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = str_nums.map(|x| x.parse::<i32>().unwrap()).collect();
    let mut result = 1;
    let mut aux = 1;

    for pos in 1..n {
        let index = pos as usize;
        if nums[index] >= nums[index - 1] {
            aux += 1;
        } else {
            if aux > result {
                result = aux;
            }

            aux = 1;
        }
    }

    if aux > result {
        result = aux;
    }

    println!("{}", result);
}
