pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    //cargo test one_hundred 注意⚠️参数只能传一个
}

/*
按名称运行测试的子集
•选择运行的测试：将测试的名称（一个或多个）作力 cargo test 的参数

按名称运行测试的子集

- ﻿选择运行的测试：将测试的名称（一个或多个）作为 cargo test 的参数
- ﻿（例子）
- ﻿运行单个测试：指定测试名

- ﻿（例子）

- ﻿运行多个测试：指定测试名的一部分（模块名也可以）

- ﻿（例子）
*/
