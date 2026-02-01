use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
// Rust 未使用传播错误的代码
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.md");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

//Rust 使用传播错误的代码
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.md")?;

//     // ? 相当于下面👇这些代码 Rust 会自动转换为 io::Error的错误类型
//     // let mut f = match f {
//     //         Ok(file) => file,
//     //         Err(e) => return Err(e),
//     //     };

//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// Rust 使用链式调用的形式
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.md")?.read_to_string(&mut s)?;
//     Ok(s)
// }

/*
    Box<dyn Error> 代表任何可能错误类型
*/

fn main() -> Result<(), Box<dyn Error>> {
    //传播错误操作
    // let result = read_username_from_file();

    let f = File::open("hello.md")?; //
    Ok(())
    //the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    //cannot use the `?` operator in a function that returns `()`
    // ? 只能用于 Result 类型
}
