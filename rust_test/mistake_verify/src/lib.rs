pub struct Guess {
    value: u32,
}

// impl Guess {
//     pub fn new(value: u32) -> Guess {
//         // if value < 1 || value > 100 {
//         //     panic!("Guess value must be between 1 and 100, got {},", value)
//         // }
//         if value < 1 {
//             panic!("Guess value must be between 1 and 100, got {},", value)
//         }
//         Guess { value }
//     }
// }

impl Guess {
    pub fn new(value: u32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {},", value)
        // }
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {},",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(0);
    }
}

/*
验证错误处理的情况
    •	测试除了验证代码的返回值是否正确，还需验证代码是否如预期的处理了发生错误的情况
    •	可验证代码在特定情况下是否发生了panic
    •	should_panic 属性（attribute）：
    ⁃	函数 panic：测试通过
    ⁃	函数没有 panic：测试失败

让should_panic更精确
    •	为 should_panic 属性添加一个可选的 expected 参数：
    ⁃	将检查失败消息中是否包含所指定的文字

     panic message: "Guess value must be greater than or equal to 1, got 0,"
 expected substring: "Guess value must be less than or equal to 100"
*/
