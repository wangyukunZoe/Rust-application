use std::io;
use rand::Rng;  //trait随机数生成器

fn main() {

    println!("Let Guess a Number!😋");

    let secret_number = rand::rng().random_range(1..101);
    println!("The Guess number is :{}", secret_number);

    let mut input_value:String = String::new();
    
    io::stdin().read_line(&mut input_value).expect("Maybe is not a right choice, Just Try Again😄");

    println!("Your guess number is🎲: {}", input_value);

}



/*
wangyukun@wangyukundeMacBook-Pro-3 GuessNumber2 % cargo build                                                                                      [0]
   Compiling cfg-if v1.0.4
   Compiling libc v0.2.180
   Compiling zerocopy v0.8.33
   Compiling getrandom v0.3.4
   Compiling rand_core v0.9.3
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.9.0
   Compiling rand v0.9.2
   Compiling GuessNumber2 v0.1.0 (/Users/wangyukun/Desktop/develop/rust/RustLearningNotes/Rust-application/GuessNumber2)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.29s


*/