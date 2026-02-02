pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    loop {
        // ...

        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // if guess < 1 || guess > 100 {
        //     println!("The sceret number will be between 1 and 100.");
        //     continue;
        // }
        let guess = Guess::new(guess);
        // 如果发生了 panic，就说明输入不符合要求

        // ...
    }

    /*
       复习
       Rust 中的 impl（implementation）关键字用于为结构体（struct）、枚举（enum）或特性（trait）定义具体实现，主要用于实现方法（Method）和特性（Trait）。它允许将函数与特定的数据类型关联，实现面向对象编程中的功能定义、数据封装和代码复用。
    */
}
