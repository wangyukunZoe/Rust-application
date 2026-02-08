pub fn add_two(a: i32) -> i32 {
    a + 2
    // a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        //相等的
        //assert_eq: 左值，右值 参数列表：期待的值，执行结果
        // assert_eq!(4, add_two(2));
        //不相等的
        assert_ne!(5, add_two(2));
    }
}
