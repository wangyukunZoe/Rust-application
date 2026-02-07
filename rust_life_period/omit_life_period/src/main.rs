struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        //因为我们不会返回任何需要引用的变量或数据，所以不需要写生命周期
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("Could not found a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
静态生命周期
    ◦	‘static 是一个特的生命周期：整个程序的持续时间。一 例如：所有的字符串字面值都拥有‘static 生命周期
    ◦	let s: &'static str = "I have a static lifetime.";
    ◦	为引用指定‘static 生命周期前要三思：
    ⁃	是否需要引用在程序整个生命周期内都存活。
*/

/*
生命周期省略的三个规则
    ◦	﻿编译器使用3个规则在没有显式标注生命周期的情况下，来确定引用的生命周期
        规则1应用于输入生命周期
    ⁃	﻿规则2、3应用于输出生命周期
    ⁃	﻿﻿如果编译器应用完 3个规则之后，仍然有无法确定生命周期的引用 -> 报错
    ⁃	﻿﻿这些规则适用于 fn 定义和impl块
    ◦	﻿规则1：每个引用类型的参数都有自己的生命周期
    ◦	﻿规则2：如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
    ◦	﻿﻿规则3：如果有多个输入生命周期参数，但其中一个是 &self 或&mut self（是方法），那么 self 的生命周期会被赋给所有的输出生命周期参数
*/

// 方法定义中的生命周期标注
/*
方法定义中的生命周期标注
•在struct上使用生命周期实现方法，语法和泛型参数的语法一样
• 在哪声明和使用生命周期参数，依赖于：
-生命周期参数是否和字段、方法的参数或返回值有关
• struct字段的生命周期名：
- 在impl 后声明
- 在 struct 名后使用
-这些生命周期是 struct 类型的一部分
•impl 块内的方法签名中：
-引用必须绑定于 struct 字段引用的生命周期，或者引用是独立的也可以
- 生命周期省略规则经常使得方法中的生命周期标注不是必须的
*/
