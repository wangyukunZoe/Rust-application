use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Let's guess a number: ");

    let secret_num = rand::rng().random_range(1..100);

    loop{
        
        let mut user_num  = String::new();

        io::stdin().read_line(&mut user_num).expect("please type a number");

        let user_num:i32 =match user_num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match user_num.cmp(&secret_num){
            Ordering::Greater => println!("Your number is bigger than the guess one, Try it again!"),
            
            Ordering::Equal => {
                println!("You hit the guess number, Congratulations!");
                break;
            }

            Ordering::Less => println!("Your number is smaller than the guess one, Try it again!")
        }
    }
}
