use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // let f = File::open("hello.md");

    // 触发错误恐慌
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?}", error);
    //     }
    // };

    //上面这段表达式可以用下面这条语句来表示    unwarp() 相当于 match 表达式的简洁表示方法
    //但是unwarp出现的错误不可以自定义
    // let _f = File::open("hello.md").unwrap();
    //expect可以很好的解决这个问题
    let _f = File::open("hello.md").expect("无法打开文件");

    // 匹配不同类型的错误
    // let _f = match f {
    //     Ok(file) => file,
    //     // 输出不同的错误类型
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.md") {
    //             // 创建成功，返回文件名，创建失败则触发恐慌
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:?}", e),
    //         },
    //         other_error => panic!("Error opening the file: {:?}", other_error),
    //     },
    // };

    // 用闭包 closure 的方式匹配错误类型
    let f = File::open("hello.md").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.md").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Error opening file: {:?}", error);
        }
    });

    // unwarp
}
