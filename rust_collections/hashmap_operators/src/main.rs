use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    //相同键值的情况下，取值最新的那个
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    for (k,v) in scores {
        println!("{} : {}", k,v);
    }

    //示例二：只有k不对应任何值的情况下，才插入 V
    let mut scores1 = HashMap::new();
    
    scores1.insert(String::from("Blue"), 10);

    // scores1.entry(String::from("Yellow")).or_insert(50);
    let e = scores1.entry(String::from("Yellow"));
    println!("{:?}", e);
    e.or_insert(50);
    scores1.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores1);

    //示例三

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    //将上面的字符串按照空格进行分割
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:#?}", map);
}