use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess a Number!🎯");

    let secert_number = rand::rng().random_range(1..100);

    // println!("the secert_number is :{}", secert_number);

    // rust中最简单的一种循环♻️
    loop{

        //但这样的循环会无限猜下去，没有退出
        println!("Please type a number:🃏 ");

        let mut user_number:String = String::new();
    
        io::stdin().read_line(&mut user_number).expect("Please type a number⛔️");
    
        //利用 match 来捕获异常
        let user_number:i32 = match user_number.trim().parse(){
            //返回异常
            Ok(num) => num,
            Err(_) => continue, //Err(_)表示不检测任何错误类型，不输出任何错误的信息值
        };
    
        match user_number.cmp(&secert_number) {
            Ordering::Less => println!("Sorry, Your number is too small, Just Try once again! 😜"),
            
            Ordering::Equal => {
                println!("Greate! you have hit the number! Congratulations!🥳 ");
                break;
            },
            
            Ordering::Greater => println!("Sorry, Your number is too big, Just Try once again! 😛"),
    
        }
    }


}
