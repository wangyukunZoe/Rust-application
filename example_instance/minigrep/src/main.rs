// cargo run xxxx xxxx.txt

// use std::env::args();     //args()返回一个迭代器 collect
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //对参数产生一个集合使用 vector

    // env::args_os(); //OsString
    // println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);
}
