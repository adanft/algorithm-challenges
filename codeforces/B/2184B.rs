use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let len: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..len {
        let s_num: i32 = it.next().unwrap().parse().unwrap();
        let k_num: i32 = it.next().unwrap().parse().unwrap();
        let m_num: i32 = it.next().unwrap().parse().unwrap();

        if k_num >= s_num {
            let num = s_num - (m_num % k_num);

            if num > 0 {
                println!("{}", num);
            } else {
                println!("0");
            }
        } else {
            if (m_num / k_num) % 2 == 0 {
                println!("{}", s_num - (m_num % k_num));
            } else {
                println!("{}", k_num - (m_num % k_num));
            }
        }
    }
}
