use std::io;
use rand::Rng;

fn main() {
    println!("Let's to Guess a number: 😄");

    let mut user_value:String = String::new();
    
    let secret_number:i32 = rand::rng().random_range(1..100);

    println!("The secret number is: {}", secret_number);

    io::stdin().read_line(&mut user_value).expect("Maybe this is not a right one, Just try another one 😄");

    println!("This is your guess number: 🎲{}", user_value);
     
}
