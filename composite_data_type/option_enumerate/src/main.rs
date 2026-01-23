
// Rust 中类似于NULL 概念的枚举
fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;

    // 在 Rust 中不可以把Option<T>中的T，直接当成T来使用
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;  ^ no implementation for `i8 + Option<i8>`
    let sum = x + y.unwrap_or(0);

    println!("sum = {}", sum);
}
