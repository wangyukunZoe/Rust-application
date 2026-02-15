use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

//函数返回了一个含有 error 的 trait 的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //读取文件
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    // println!("With text:\n{}", contents);
    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

//这里用生命周期是因为2个参数的生命时常不一样！第一个query，查询结束后就释放了，但是第二个不能，因为还要//返回结果给用户
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
