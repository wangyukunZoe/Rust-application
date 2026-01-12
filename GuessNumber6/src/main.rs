use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    
    println!("Let's Guess a number🎯");

    let secret_num:i32 = rand::rng().random_range(1..100);
    

    loop{

        let mut user_num:String = String::new();

        io::stdin().read_line(&mut user_num).expect("Please type a number⛔️");

        let user_num:i32 = match user_num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
         };

         match user_num.cmp(&secret_num){
            Ordering::Greater => println!("Your guess number is too Big! Try it again!😬"),
            
            Ordering::Equal => {
                println!("You have HIT the guess number, Congratulations!🥳🎁");
                break;
            },

            Ordering::Less => println!("Your guess number is too Small! Try it again!😶‍🌫️"),
         }
    }
    

}
