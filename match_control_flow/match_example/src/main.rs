#[warn(unused)]
#[warn(dead_code)]
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/*
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
*/

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {   // 绑定值的模式匹配
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let coin:u8 = value_in_cents(Coin::Penny);
    println!("coin Penny💵:{}", coin);

    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));
}
