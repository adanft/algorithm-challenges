use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut str_nums = input.split_whitespace();

    let _ = str_nums.next().unwrap();
    let mut nums: Vec<i32> = str_nums.map(|x| x.parse::<i32>().unwrap()).collect();

    nums.sort_by(|a, b| b.cmp(a));

    let total: i32 = nums.iter().sum();

    let mut result = 0;
    let mut aux = 0;

    for num in nums {
        aux += num;

        if aux > total - aux {
            result += 1;
            break;
        }

        result += 1;
    }

    println!("{}", result);
}
