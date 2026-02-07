fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

// 这些引用必须拥有相同的生命周期
//就好比你爸妈在家里房间里，你想找他们其中一个借钱，但只要房间熄灯了，就说明肯定爸妈有一个睡了，不管找谁都别敲门了。'a就是那个提醒的灯

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
