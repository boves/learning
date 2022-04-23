use std::io;
// use std::env; // only needed for set_var(...)
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    // env::set_var("RUST_BACKTRACE","1");
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101); //immutable

    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    // let mut guess = String::new(); // mutable

    loop{
        
        let mut guess = String::new(); // mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
       
       // .expect("Please type a number!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                                  println!("You win!");
                break;
            }
        }
    }
}
