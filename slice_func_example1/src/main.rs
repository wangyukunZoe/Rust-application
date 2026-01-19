fn main() {
    let s = String::from("Hello world");

    let _hello = &s[0..5];   //引用的是字符串的一部分，包含索引 0，但不包含索引 5
    // let _world = &s[6..11];  // 范围，左闭右开
    //let _world = &s[6..s.len()];  // 范围，左闭右开
     let _world = &s[6..];  // 范围，左闭右开

     let _whole = &s[..];    //指向整个字符串切片

    println!("{}-{}",  _hello, _world);
    println!("{}",  _whole);

}
