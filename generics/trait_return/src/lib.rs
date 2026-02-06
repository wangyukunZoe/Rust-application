use std::fmt::{Debug, Display};

pub trait Summary {
    //默认方法的签名
    //fn summarize_author(&self) -> String;

    // fn summarize(&self) -> String;   // 我们可以做一个默认的实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    //默认实现的重写的实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// item 实现了 Summary trait这个类型
// pub fn notify1(item1: impl Summary, item2: impl Summary) {
//     println!("Breaking news! {}", item1.summarize())
// }

pub fn notify1(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup championship"),
        content: String::from(
            "The Pittsburgh penguins once again are the best hockey team in the NHL.",
        ),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh,PA,USA"),
    }
    //但这种情况不能处理两种值，可能会导致返回类型🔙不止一个
    //这在 Rust 中是不被允许的
}

// // trait bound，下面是这个语法的语法糖🍭写法
// pub fn notify<T: Summary + Display>(item1: T, item2: T) -> String {
//     println!("Breaking news! {}", item1.summarize());
//     item1.summarize()
// }

// //使用 where 字句来简化 trait 的约束
// pub fn notify2<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
//     println!("Breaking news! {}", a.summarize());
//     a.summarize()
// }

// pub fn notify3<T, U>(a: T, b: U) -> String
// where
//     T: Summary + Display,
//     U: Clone + Debug,
// {
//     println!("Breaking news! {}", a.summarize());
//     a.summarize()
// }
