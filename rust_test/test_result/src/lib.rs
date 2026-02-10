#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

/*
在测试中使用 Result<T,E>
*  无需 panic，可使用 Result<T, E> 作次返回类型编写测试：
-  返回Ok：测试通过
-  返回Err：测试失败
*  （例子）
•注意：不要在使用 Result<T, E> 编写的测试上标注 #［should_panic］

*/
