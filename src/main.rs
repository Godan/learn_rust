extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    println!("数あてゲーム");
    


    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("行の読み込みに失敗しました");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
           
        println!("予想: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal =>{
                println!("You win");
                println!("number is {}", secret_number);
                break;
            } 
        }
    }
}
