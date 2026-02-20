use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

//函数返回了一个含有 error 的 trait 的类型
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //读取文件
    let contents = fs::read_to_string(config.filename)?;
    let _results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // for line in search(&config.query, &contents) {
    //     println!("{}", line);
    // }
    for line in _results {
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
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //将大写或小写字符串一律转换成小写字符串
    let mut results = Vec::new();
    let query = query.to_lowercase(); //这个是拥有所有权的，因为他会创建一个新的数据

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
    // TEST quota :// vec![]
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}

/*
windows 下使用环境变量
windows 使用 cmd 用命令 set CASE_INSENSITIVE=1 && cargo run to poem.txt
windows 在cmd窗口   set CASE_INSENSITIVE=1 | cargo run duct poem.txt
*/
