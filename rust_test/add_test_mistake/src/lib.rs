pub fn greeting(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "\x1b[31mGreeting did not contain name, value was `{}`\x1b[0m",
            result
        );
        // assert!([断言测试功能语句], [添加自定义错误信息])
        //可以比如红色：\x1b[31mGreeting did not contain name, value was `{}`\x1b[0m
    }
}

/*
thread 'tests::greeting_contains_name' (206962) panicked at src/lib.rs:12:9:
assertion failed: result.contains("Carol")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/
