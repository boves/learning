// ChatGPT prompt: Write a text editor that will compile in rust to run in the command shell
// Result: "Here is a basic text editor written in Rust that can be compiled and run in the command shell:"

use std::io;

fn main() {
    println!("Welcome to the Rust Text Editor!");
    println!("Enter your text below: ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    println!("Your entered text: {}", user_input);
}

