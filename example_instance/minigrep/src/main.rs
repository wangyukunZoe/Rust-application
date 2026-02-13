// cargo run xxxx xxxx.txt

// use std::env::args();     //args()返回一个迭代器 collect
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); //对参数产生一个集合使用 vector
    let config = Config::new(&args);
    //读取文件
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file.");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }

// let (query, filename) = parse_config(&args);

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];

//     (query, filename)
// }

// env::args_os(); //OsString
// println!("{:?}", args);

// println!("Search for {}", query);
// println!("In file {}", filename);

/*
二进制程序关注点分离的指导性原则
*  将程序拆分为 main.rs和 lib.rs，将业务逻辑放入 lib.rs
*  当命令行解析逻辑较少时，将它放在 main.rs 也行
*  当命令行解析逻辑变复杂时，需要将它从 main.rs 提取到 lib.rs

*/
