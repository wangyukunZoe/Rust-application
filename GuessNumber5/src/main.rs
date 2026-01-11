use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess a Number!🎯");

    let secert_number = rand::rng().random_range(1..100);

    println!("the secert_number is :{}", secert_number);

    let mut user_number:String = String::new();

    io::stdin().read_line(&mut user_number).expect("Please type a number⛔️");

    let user_number:i32 = user_number.trim().parse().expect("Please type a integer number");

    match user_number.cmp(&secert_number) {
        Ordering::Less => println!("Sorry, Your number is too small, Just Try once again! 😜"),
        
        Ordering::Equal => println!("Greate! you have hit the number! Congratulations!🥳 "),
        
        Ordering::Greater => println!("Sorry, Your number is too big, Just Try once again! 😛"),

    }

}
