pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4); //assert_equal判断是否相等
    }

    // 测试失败的例子
    #[test]
    fn another() {
        panic!("Make this test fail")
    }
}
// 测试中遇到 panic 恐慌就是测试失败

/*
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----

thread 'tests::another' (205244) panicked at src/lib.rs:18:9:
Make this test fail
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

*/
