// fn main() {
//     let number = 3;
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }

// }


// fn main(){
    
//     let number = 6;

//     //如果你使用了多个 else if 那么最好用match重构一下，防止代码的凌乱
//     // 实际上,在 rust 中，match 有异常处理的作用
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3 or 2");
//     }

// }

// if-else with match expression
fn main(){
    
    let number = 6;

    //如果你使用了多个 else if 那么最好用match重构一下，防止代码的凌乱
    // 实际上,在 rust 中，match 有异常处理的作用
    match number {

        number if number % 4 == 0 => println!("number is divisible by 4"),

        number if number % 3 == 0  => println!("number is divisible by 3"),

        number if number % 2 == 0 => println!("number is divisible by 2"),

        _ =>  println!("number is not divisible by 4, 3 or 2")
    }
}

/*
 val if val % 3 == 0  => println!("number is divisible by 3"),

        val if val % 2 == 0 => println!("number is divisible by 2"),
*/

// fn main(){
//     let condition = true;

//     let number = if condition { 5 } else { 6 }; //if-else 内部返回值的类型一定要一致，尤其在没有定义类型的情况下（需要进行类型推断的情况下）

//     println!("The value of number is: {}", number); //The value of number is: 5
// }