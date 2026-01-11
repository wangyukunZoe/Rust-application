use std::io;
use rand::Rng;

fn main() {
    println!("Guess a Number!🎯");

    let secert_number = rand::rng().random_range(1..100);

    let mut user_number:String = String::new();

    io::stdin().read_line(&mut user_number).expect("Please type a number⛔️");

}
