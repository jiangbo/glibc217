use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("请输入一个数字：");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if let Ok(input) = input.trim().parse::<i32>() {
            match input.cmp(&num) {
                Ordering::Less => println!("太小了"),
                Ordering::Greater => println!("太大了"),
                Ordering::Equal => {
                    println!("猜中了");
                    break;
                }
            }
        }
    }
}
