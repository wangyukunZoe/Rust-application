fn main() {
    let string1 = String::from("abcd");
    let result;
    // let string2 = String::from("xyz");   //string2 这里就没有问题了
    {
        let string2 = String::from("xyz"); //string2 这里就会触发一个异常！
        result = longest(string1.as_str(), string2.as_str());
    }
    //另外string2出了括号作用域就消失了🫠，要是在被引用会引发悬垂指针的问题
    println!("The longest string is {}", result);
    /*
    str1的生命周期是2 -> 9，str2的生命周期是5 -> 7
    而result的生命周期是3 -> 9，不在 'a 的范围内，所以Bong！
    所以 'a 表示的生命周期是两者的生命周期的交集，也就是5 -> 7
    结果生命周期等于传入的两个生命周期中最短的那一个
     */
}

// 这些引用必须拥有相同的生命周期
//就好比你爸妈在家里房间里，你想找他们其中一个借钱，但只要房间熄灯了，就说明肯定爸妈有一个睡了，不管找谁都别敲门了。'a就是那个提醒的灯

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
