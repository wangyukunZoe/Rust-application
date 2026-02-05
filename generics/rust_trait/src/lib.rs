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
