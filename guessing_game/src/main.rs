use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..5);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Oh no!, your guess is lesser than the secret number"),
            Ordering::Greater => println!("Oh no!, your guess is greater than the secret number"),
            Ordering::Equal => {
                println!("Yay! You guessed it right!");
                break;
            }
        }
    }
}
