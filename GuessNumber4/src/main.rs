use std::{cmp::Ordering, io};   //Ordering 是一种枚举类型，拥有三个比较关键字
use rand::Rng;

fn main() {
    
    println!("Let's try to guess a number:😄");

    println!("Generate a random number");
    let secert_number = rand::rng().random_range(1..100);   //i32 u32 i64

    println!("This is the random number: {}", secert_number);

    let mut user_number:String = String::new();

    io::stdin().read_line(&mut user_number)
    .expect("Maybe this is not the right one. Try it again! ☕️");

    println!("This is your number: 🎲 {}", user_number);

    //将字符串类型转换成整数类型的第一种方式
   // let user_number: i32 = user_number.trim().parse().expect("Please type a number!");

   //.trim() 删除字符串内部的空格   包括 '\n等转义字符
   //shadow 基于安全🔐规则，允许我们复用新的变量名
    let user_number:i32 = user_number.trim().parse().expect("Please type a number.");

    //match 表达式可以根据cmp方法用来匹配
    //现在无法进行比较，因为需要将字符串类型转换成整数类型
    match user_number.cmp(&secert_number){
        Ordering::Greater => println!("Your guess number is bigger than the secert one, You can try it again~😜😶‍🌫️"),  //arm

        Ordering::Less => println!("Your guess number is smaller than the secert one, You can try it again~🙄"),

        Ordering::Equal => println!("Great!!! Congratulations!🎁"),

    }


    //第二种比较方法
    /* 
        if user_number > secert_number {
            println!("Your guess number is bigger than the secert one, You can try it again~😜");
        } else if user_number < secert_number {
            println!("Your guess number is smaller than the secert one, You can try it again~😜");
        } else if user_number == secert_number {
            println!("Great!!! ");
            println!("Your number is: 🎲 {}, and the secert number 
            is {}, congratulations!🎁", user_number, secert_number);
        }

    */
}
