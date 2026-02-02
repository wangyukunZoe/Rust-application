use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // 这段代码的 Result绝对是 true
    // catch_unwind(try_fn, data, catch_fn)

    loop {
        // ...

        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The sceret number will be between 1 and 100.");
            continue;
        }

        // ...
    }
}

/*
    # 错误处理
*/
