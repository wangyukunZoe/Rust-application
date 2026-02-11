#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(5, 1 + 1 + 1 + 1 + 1);
    }
}

/*
忽略某些测试， 运行剩余测试
ignore 属性(attribute)
cargo test -- --ignored  运行☁️被忽略的测试

*/
