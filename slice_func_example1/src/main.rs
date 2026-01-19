fn main() {

    //指向二进制程序特定的一个切片
    let s = "hello world";

    let s = String::from("Hello world");

    let _hello = &s[0..5];   //引用的是字符串的一部分，包含索引 0，但不包含索引 5
    // let _world = &s[6..11];  // 范围，左闭右开
    //let _world = &s[6..s.len()];  // 范围，左闭右开
     let _world = &s[6..];  // 范围，左闭右开

     let _whole = &s[..];    //指向整个字符串切片

    println!("{}-{}",  _hello, _world);
    println!("{}",  _whole);

    let a = [1,2,3,4,5];
    let slice = &a[1..3];   // 这个切片存储一个指针，指向上面数组中的起始位置，在此之上还存储了一个长度
}
