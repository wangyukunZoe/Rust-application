use demo::NewsArticle;
use demo::Summary;
use demo::Tweet;

fn main() {
    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize())

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup championship"),
        content: String::from(
            "The Pittsburgh penguins once again are the best hockey team in the NHL.",
        ),
        author: String::from("Iceburgh"),
        location: String::from("Pittsburgh,PA,USA"),
    };

    println!("1 new tweet: {}", article.summarize())
}
