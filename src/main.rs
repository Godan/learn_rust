extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

const NUM: i32 = 100000;

fn game() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("数あてゲーム");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("予想: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                println!("number is {}", secret_number);
                break;
            }
        }
    }
}

fn sample() {
    // game();
    // const
    println!("The value of NUM is :{}", NUM);

    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn main() {
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is {}", y);
    println!("The value of tup.0 is {}", tup.0);
    println!("The value of tup.1 is {}", tup.1);
    println!("The value of tup.2 is {}", tup.2);

    let a = [1, 2, 3, 4, 5];
    let index = rand::thread_rng().gen_range(5, 5);
    let element = a[index];
    println!("{}", element);
}
