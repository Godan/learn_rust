extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("number is {}", secret_number);
    println!("数あてゲーム");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("行の読み込みに失敗しました");
    println!("予想: {}", guess);
}
