pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// 私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] //单元测试
mod tests {
    use crate::internal_adder;

    #[test]
    fn it_works() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
/*
Rust 允许测试私有函数
*/
