use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number : u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    /*
    2 Programing a gessing game
      Comparing the Guess to the Secret Number       
    */ 


    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("you wîn"),
        Ordering::Greater => println!("to big"),
        Ordering::Less => println!("to small")
    }
}